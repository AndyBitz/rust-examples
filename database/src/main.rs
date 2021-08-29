use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::time::Instant;

mod page;
mod row;
mod page_reader;

fn main() {
    if !std::path::Path::new("data.txt").exists() {
        create_data_pages();
    }
    
    if !std::path::Path::new("index.txt").exists() {
        create_index();
    }

    let search_age = 32;
    full_search(search_age);
    binary_search(search_age);
}

fn binary_search(age_match: i32) {
    let now = Instant::now();

    let mut count = 0;

    let mut index_file = File::open("index.txt").unwrap();

    // 0xc is the length of each tuple
    let file_length = index_file.metadata().unwrap().len();
    let index_items = file_length / 0xC;

    let mut start_index = index_items / 2;

    loop {
        index_file.seek(SeekFrom::Start(start_index * 0xC)).unwrap();

        let mut data = [0x0u8; 0x4];
        index_file.read_exact(&mut data).unwrap();

        let age = i32::from_le_bytes(data);

        if age > age_match {
            start_index = start_index / 2;
        } else if age < age_match {
            start_index = start_index + (start_index / 2);
        } else {
            break;
        }
    }

    let mut found_pages = Vec::<u64>::new();

    // Walk down // TODO
    for index in (0..start_index).rev() {
        index_file.seek(SeekFrom::Start(index * 0xC)).unwrap();

        let mut age_bytes = [0x0u8; 0x4];
        let mut page_bytes = [0x0u8; 0x8];

        index_file.read_exact(&mut age_bytes).unwrap();
        index_file.read_exact(&mut page_bytes).unwrap();

        let age = i32::from_le_bytes(age_bytes);
        let page = u64::from_le_bytes(page_bytes);

        if age == age_match {
            found_pages.insert(0, page);
        } else {
            break;
        }
    }

    // Walk up // TODO
    index_file.seek(SeekFrom::Start(start_index * 0xC)).unwrap();
    for _index in start_index..index_items {
        let mut age_bytes = [0x0u8; 0x4];
        let mut page_bytes = [0x0u8; 0x8];
        
        index_file.read_exact(&mut age_bytes).unwrap();
        index_file.read_exact(&mut page_bytes).unwrap();

        let age = i32::from_le_bytes(age_bytes);
        let page = u64::from_le_bytes(page_bytes);

        if age == age_match {
            found_pages.push(page);
        } else {
            break;
        }
    }

    let mut page_iterator = page_reader::PageReader::new(File::open("data.txt").unwrap());
    let row_size = std::mem::size_of::<row::Row>();

    let mut processed_pages = Vec::<u64>::new();
    let mut last_proccessed = u64::MAX;

    // Assumes that `found_pages` is ordered.
    for page_offset in found_pages {
        if last_proccessed == page_offset {
            continue;
        }
        last_proccessed = page_offset;
        processed_pages.push(page_offset);

        page_iterator.skip_to_page(page_offset as usize);

        let page = match page_iterator.read_current() {
            None => break,
            Some(r) => r,
        };

        for i in 0..(usize::from(page.data_length) / row_size) {
            let start = i * row_size;
            let end = start + row_size;

            let mut data = [0x0u8; 516];
            data.copy_from_slice(&page.data[start..end]);
            let row = row::Row::from_bytes(&data);

            if row.age == age_match {
                count += 1;
            }
        }
    }

    println!("\nBinary Search:");
    println!("Found {} users with the age of {} in {} ms", count, age_match, now.elapsed().as_millis());
    println!("Went through {} pages", processed_pages.len());
}

fn full_search(age_match: i32) {
    let now = Instant::now();

    let mut count = 0;

    let data_file = File::open("data.txt").unwrap();
    let page_iterator = page_reader::PageReader::new(data_file);
    let row_size = std::mem::size_of::<row::Row>();
    let mut processed_pages = Vec::<u64>::new();
    let mut pages_with_result = std::collections::HashSet::<u64>::new();

    for page in page_iterator {
        processed_pages.push(page.offset);

        for i in 0..(usize::from(page.data_length) / row_size) {
            let start = i * row_size;
            let end = start + row_size;

            let mut data = [0x0u8; 516];
            data.copy_from_slice(&page.data[start..end]);
            let row = row::Row::from_bytes(&data);

            if row.age == age_match {
                count += 1;
                pages_with_result.insert(page.offset);
            }
        }
    }

    println!("\nFull Search:");
    println!("Found {} users with the age of {} in {} ms", count, age_match, now.elapsed().as_millis());
    println!("Went through {} pages (pages with results {})", processed_pages.len(), pages_with_result.len());
}

/**
 * Call this function to create an example data.txt file
 * with randomly generated data that can be worked with.
 */
fn create_data_pages() {
    let data_file = File::create("data.txt").unwrap();
    let mut buffer = BufWriter::with_capacity(0xffff, data_file);

    let now = Instant::now();
    let rows: u64 = 100_000;

    for elem in 0u64..rows {
        let mut page = page::Page::new(elem);

        for _i in 0..100 {
            let id: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();

            let username_length = rand::thread_rng().gen_range(2..22);
            let username: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(username_length)
                .map(char::from)
                .collect();

            let age = rand::thread_rng().gen_range(1i32..99i32);

            let row = row::Row::new(id, username, age);
            page.put(&row.serialize());
        }

        buffer.write(&page.serialize()).unwrap();
    }

    println!("Done in {} seconds", now.elapsed().as_secs());
}

fn create_index() {
    let now = Instant::now();

    let data_file = File::open("data.txt").unwrap();
    let index_file = File::create("index.txt").unwrap();
    let mut write_buffer = BufWriter::with_capacity(0xffff, index_file);

    let page_iterator = page_reader::PageReader::new(data_file);
    let row_size = std::mem::size_of::<row::Row>();

    let mut mem_index = Vec::<(i32, u64)>::new();

    // TODO: Can be improved by using pages for the index.
    for page in page_iterator {
        for i in 0..(usize::from(page.data_length) / row_size) {
            let start = i * row_size;
            let end = start + row_size;

            let mut data = [0x0u8; 516];
            data.copy_from_slice(&page.data[start..end]);

            let row = row::Row::from_bytes(&data);
            mem_index.push((row.age, page.offset));
        }
    }

    mem_index.sort();

    for (age, page) in mem_index {
        write_buffer.write(&age.to_le_bytes()).unwrap();
        write_buffer.write(&page.to_le_bytes()).unwrap();
    }

    println!("Created index in {} seconds", now.elapsed().as_secs());
}
