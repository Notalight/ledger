use std::fs::File;
use std::path::Path;

use polars::prelude::Result as PolarResult;
use polars::prelude::*; //{CsvReader, DataType, Field, Result as PolarResult, Schema, DataFrame,};
                        //use polars_core::prelude::*;
use polars::frame::DataFrame;

use chrono::{Datelike, NaiveDate, NaiveDateTime, TimeZone, Utc, Weekday};

use serde::{Deserialize, Serialize};

//use yahoo_finance::{history, Interval, Timestamped};
use std::time::{Duration, UNIX_EPOCH};
use yahoo_finance_api as yahoo;

// use std::fs;
// use std::net::SocketAddr;
// use std::error::Error;
use std::io;
use std::io::prelude::*;

mod model;
use crate::model::{Portfolio, load_portfolio};
//pub use model::Portfolio ;

#[tokio::main]
async fn main() {
    // // A reference with 'static lifetime:
    // let _s: &'static str = "hello world";
    // Loading Portfolio


    // here iris
    let ifile = "iris.csv";
    // shape info
    deal_with_shape(&ifile);
    // columns info
    deal_with_columns(&ifile);
    // concatenate dataframe
    deal_with_stacks(&ifile);
    // do math on this
    deal_with_apply(&ifile);

    let start = NaiveDate::from_ymd_opt(2015, 3, 14).unwrap();
    let end = NaiveDate::from_ymd_opt(2016, 3, 14).unwrap();
    //let start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    //let end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);

    // let mut count = 0;
    // for (idx, d) in NaiveDate::from_ymd(2016, 2, 27)
    //     .iter_days()
    //     .take(4)
    //     .enumerate()
    // {
    //     count += 1;
    //     //println!("Date({})= {}", count, d);
    // }

    let vec_date = create_date_range(start, end);
    //let chunk_date = DateChunked::new_from_naive_date("test", &vec_date);
    let series_v1 = DateChunked::new_from_naive_date("date", &vec_date).into_series();
    //let chunk_date: ChunkedArray<NaiveDate> = ChunkedArray::new_from_aligned_vec("test",vec_date) ;
    // Series can also be collected from iterators
    //let from_iter: Series = chunk_date.into_iter().collect();
    //let date_range = Series::new("test2", vec_date.iter().collect()) ;
    //println!(date_range);
    //let df = polars::functions::frame::date_range(low=datetime(2021, 1, 1), high=datetime(2021, 12, 31), interval="1d", name="time").to_frame() ;
    //let chunked_array = from_iter.i32().unwrap();
    println!("{}", series_v1);
    //println!("{}", from_iter.head(Some(5)));

    //let range = polars::prelude::range(start, end);

    // // retrieve 6 months worth of data for Apple
    // let data = history::retrieve_interval("AAPL", Interval::_ytd)
    //     .await
    //     .unwrap();

    // // print the date and closing price for each day we have data
    // for bar in &data {
    //     println!(
    //         "Apple hit an intraday high of ${:.2} on {}.",
    //         bar.close,
    //         bar.datetime().format("%b %e %Y")
    //     );
    // }
    //format("%b %e %Y")

    //
    //
    //

    let portfolio = load_portfolio("portfolio_temp.json".to_string());

    let mut file = File::open("portfolio_temp.json").unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    println!("{}", &buffer);

    // Convert the JSON string back to a Point.
    let deserialized: Portfolio = serde_json::from_str(&buffer).unwrap();

    println!(
        "{} - {}",
        deserialized.name,
        deserialized.last_update.unwrap().to_string()
    );
    for asset in deserialized.assets {
        println!(
            "Asset : {} : {} €",
            asset.description, asset.estimation_price
        );
    }

    //
    //
    //

    //let provider = yahoo::YahooConnector::new();
    //let yf_start = Utc.ymd(2020, 1, 1).and_hms_milli(0, 0, 0, 0);
    //let yf_end = Utc.ymd(2020, 1, 31).and_hms_milli(23, 59, 59, 999);

    //let resp = provider.get_quote_history("AAPL", yf_start, yf_end).await.unwrap();
    //let quotes = resp.quotes().unwrap();
    //println!("Apple's quotes in January: {:?}", quotes);
}

fn create_date_range(start: NaiveDate, end: NaiveDate) -> Vec<NaiveDate> {
    let mut vec = Vec::new();
    let mut count = 0;

    let since = NaiveDate::signed_duration_since;
    let days = since(end, start).num_days();

    println!("Number of days : {}", days);

    for (idx, d) in start.iter_days().take(days.try_into().unwrap()).enumerate() {
        count += 1;
        vec.push(d);
        println!("Date({})= {}", count, d);
    }

    return vec;
}

fn display(v: &Vec<i32>) {
    println!("inside display {:?}", v);
}

// do not return anything from this function
pub fn deal_with_shape<P: AsRef<Path>>(path: P) -> () {
    /* Example function to retrieve shape info from a dataframe */
    let df = read_csv(&path).unwrap();
    // shape
    // reming {:#?} otherwise error ^^^^^ `(usize, usize)` cannot be formatted with the default formatter
    let shape = df.shape();
    println!("{:#?}", shape);
    // schema
    println!("{:#?}", df.schema());
    // dtypes
    println!("{:#?}", df.dtypes());
    // or width and height
    let width = df.width();
    println!("{}", width);
    let height = df.height();
    println!("{}", height);
}

pub fn read_csv<P: AsRef<Path>>(path: P) -> PolarResult<DataFrame> {
    /* Example function to create a dataframe from an input csv file*/
    let file = File::open(path).expect("Cannot open file.");

    CsvReader::new(file).has_header(true).finish()
}

// do not return anything from this function
pub fn deal_with_columns<P: AsRef<Path>>(path: P) -> () {
    /* Examples to deal with column and column names and enumerate */
    let df = read_csv(&path).unwrap();
    // column functions
    let columns = df.get_columns(); // you can do for column in columns{}
    let columname = df.get_column_names();

    // example like Python for i, val in enumerate(list, 0):
    for (i, column) in columns.iter().enumerate() {
        println!("{}, {}", column, columname[i]);
    }
}

// do not return anything from this function
pub fn deal_with_stacks<P: AsRef<Path>>(path: P) -> () {
    /* Stack, often happens to stack multiple dataframes together*/
    println!("Read the same dataframe twice");
    let df = read_csv(&path).unwrap();
    let df2 = read_csv(&path).unwrap();
    println!("Vertical stac the two dataframes");
    let mut df3 = df.vstack(&df2).unwrap(); // mut --> so we can change this dataframe later
    println!("{}, {:#?}", df3.head(Some(5)), df3.shape());
    // get column
    println!("Get a column");
    let sepal_length = df3.column("sepal.length").unwrap();
    println!("{}", sepal_length);
    println!("{:#?}", sepal_length.len());

    // drop columns
    println!("Drop a column");
    let sepal_length = df3.drop_in_place("sepal.length").unwrap(); // inplace
                                                                   // this commands return a Series
    println!("{}", df3.head(Some(5)));
    // drop_nulls() to drop NaN
    //let df4 = df3.drop("sepal.length"); // if we don't want a mut dataframe df3
    println!("Insert a series in a dataframe as a new column");
    let _df4 = df3.insert_at_idx(0, sepal_length).unwrap();
    println!("{}", _df4.head(Some(5)));
}

pub fn deal_with_apply<P: AsRef<Path>>(path: P) -> () {
    /* Apply is one of the key functions in pandas*/
    let mut df = read_csv(&path).unwrap();
    // apply an operation or a function/closure
    println!("Add 1 to first column");
    df.apply_at_idx(0, |s| s + 1);
    println!("{}", df.head(Some(5)));
    // compute the log transform of a column and learn to play with series and chunked arrays
    let log10_series = numb_to_log(&mut df);
    // insert the column
    println!(" log 10 of sepal length");
    df.with_column(log10_series.unwrap());
    println!("{}", df.head(Some(5)));
    // can we log transform throught apply_at_idx?
    df.apply_at_idx(0, |s| s.f64().unwrap().apply(|t| t.log10()));
    println!("{}", df.head(Some(5)));
}

// Return a series
fn numb_to_log(in_df: &mut DataFrame) -> PolarResult<Series> {
    // do with a series  unwrap to have Series, .f64().uwrap() to retrieve a chunked array
    let to_log10_column = in_df
        .drop_in_place("sepal.length")
        .unwrap()
        .rename("log10.sepal.length")
        .f64()
        .unwrap() // create chunked array
        //        .cast::<Float64Type>() // here we have apply
        //        .unwrap() // unwrap because we have Result<>
        .apply(|s| s.log10());

    let series10 = to_log10_column.into_series(); // reconvert into a series

    // return the column
    println!("{}", series10);
    Ok(series10)
}
