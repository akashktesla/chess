#![allow(warnings)]
use std::sync::{Arc,RwLock,Mutex};
use std::mem::drop;
use chrono::prelude::*;
use regex::Regex;
use regex::CaptureMatches;
use rustpp::fs::{write_file,read_file};

fn main(){

}

#[derive(Debug)]
pub struct Database{
    pub data:Arc<RwLock<String>>,
}

impl Database{

    pub fn new()->Database{
        let data = Arc::new(RwLock::new(String::from("")));
        Database{
            data
       }
    }

    pub fn insert_data(&self,data:&String){
        let mut lock = self.data.write().unwrap();
        lock.push_str(data);
        lock.push_str("\n");
        drop(lock);
    }
    
    pub fn insert_data_unique(&self,data:&String){
        let mut lock = self.data.read().unwrap();
        let flag = (*lock).contains(&format!("{}\n",data));
        // println!("data: {:?}",*lock);
        // println!("flag: {}",flag);
        drop(lock);
        if !flag{
            self.insert_data(data);
        }
    }
    
    pub fn read_data(&self,query:&String)->Vec<String>{
        let re = Regex::new(query).unwrap();
        let lock = self.data.read().unwrap();
        let mut returns:Vec<String> = Vec::new();
        for i in re.captures_iter(&*lock){
            let temp = format!("{}",i.get(0).unwrap().as_str());
            println!("{}",temp);
            returns.push(temp);
        }
        returns.sort();
        returns.dedup();
        returns
        }

    pub fn delete_data(&self,data:&String){
        let mut lock = self.data.write().unwrap();
        (*lock).replace(data,"");
    }

    pub fn save_database(&self,path:&String){
        let data = format!("{}",*self.data.read().unwrap());
        write_file(path,&data);
    }

    pub fn load_database(&self,path:&String){
        let data = read_file(path);
        let mut lock = self.data.write().unwrap();
        lock.push_str(&data);
        drop(lock);
    }

    pub fn print_data(&self){
        let lock = self.data.read().unwrap();
        println!("{}",*lock); 
        drop(lock);
    }
    pub fn clear(&self){
        let mut lock = self.data.write().unwrap();
        *lock = String::from("");
        drop(lock);
    }
}

