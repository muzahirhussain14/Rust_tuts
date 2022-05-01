// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

#[derive(Debug)]
struct Names <'a> {
    names: Vec<&'a str>,
}

#[derive(Debug)]
struct Titles <'a> {
    titles: Vec<&'a str>
}

fn main() {

    let mut title = Titles {
        titles: vec![]
    };

    let data: Vec<&str> = MOCK_DATA.split('\n').skip(1).collect();
    let names: Vec<_> = data.iter().filter_map(|line| line.split(',').nth(1)).collect();
    let titles: Vec<_> = data.iter().filter_map(|line| line.split(',').nth(4)).collect();

    let names = Names {
        names
    };
    let titles = Titles {
        titles
    };

    //println!("Data: {:?}", names);
    //println!("Data: {:?}", titles);

    let names_titles: Vec<_> = names.names.iter().zip(titles.titles.iter()).collect();
    println!("\n************* DATA *************\n");
    
    for (name, title) in names_titles {
        println!("Name: {}, Title: {}", name, title);
    }
}
