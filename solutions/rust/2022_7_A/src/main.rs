extern crate support;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    file_sizes: usize,
    total_size: Option<usize>,
    childs: Vec<Rc<RefCell<Dir<'a>>>>,
    parent: Option<Weak<RefCell<Dir<'a>>>>
}

impl<'a> Dir<'a> {
    fn from_listing(name: &'a str, listing: Vec<ListResult<'a>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { 
            name, 
            total_size: None,
            parent: None,
            childs: vec![], 
            file_sizes: listing.iter().map(|x| match x {
                ListResult::Directory(_) => 0,
                ListResult::File(_, s) => *s
            }).sum()
        }))
    }

    fn add_child(parent: Rc<RefCell<Self>>, child: Rc<RefCell<Self>>) {
        (*parent.borrow_mut()).childs.push(child.clone());
        (*child.borrow_mut()).parent = Some(Rc::downgrade(&parent))
    }

    fn total_size(node: Rc<RefCell<Self>>) -> usize {
        if (*node.clone().borrow_mut()).total_size.is_some() {
            (*node.clone().borrow_mut()).total_size.unwrap()
        } else {
            let s = (*node.clone().borrow_mut()).file_sizes;
            let c = (*node.clone().borrow_mut()).childs.iter().map(|c| Dir::total_size(c.clone())).sum::<usize>();

            (*node.clone().borrow_mut()).total_size = 
                Some(s + c);
            s + c
        }
    }

    fn task(node: Rc<RefCell<Self>>) -> usize {
        let s = Dir::total_size(node.clone());
        let c = (*node.clone().borrow_mut()).childs.iter().map(|c| Dir::task(c.clone())).sum::<usize>();
        if s < 100000 {
            s + c
        } else {
            c
        }
    }
}

#[derive(Debug)]
enum Command<'a> {
    ChangeDirectory(&'a str),
    ListDirectory(Vec<ListResult<'a>>)
}

#[derive(Debug)]
enum ListResult<'a> {
    Directory(&'a str),
    File(&'a str, usize)
}

fn main() {
    let commands = support::test_data!()
    .split("$ ")
    .skip(1) // The delimer is at the beginning
    .map(|x| {
        let l = x.trim().lines().collect::<Vec<&str>>();
        if l.len() == 1 {
            Command::ChangeDirectory(l[0].split(' ').nth(1).unwrap())
        } else {
            Command::ListDirectory(l.into_iter().skip(1).map(|x| 
                {
                    let mut x = x.split(' ');
                    let (a, b) = (x.next().unwrap(), x.next().unwrap());
                    if a == "dir" {
                        ListResult::Directory(b)
                    } else {
                        ListResult::File(b, a.parse::<usize>().unwrap())
                    }
            }).collect())
        }
    })
    .collect::<Vec<Command>>();

    let mut pwd: Option<&str> = None;
    let mut root: Option<Rc<RefCell<Dir>>> = None;
    let mut dir: Option<Rc<RefCell<Dir>>> = None;
    for cmd in commands {
        match cmd {
            Command::ChangeDirectory("..") => {
                assert!(dir.is_some());
                dir = Some((*dir.unwrap().borrow()).parent.as_ref().unwrap().clone().upgrade().unwrap());
                pwd = None
            }
            Command::ChangeDirectory(dir) => {
                assert!(pwd.is_none());
                pwd = Some(dir)
            },
            Command::ListDirectory(listing) => {
                assert!(pwd.is_some());
                let tmp = Dir::from_listing(pwd.unwrap(), listing);
                match dir {
                    Some(dir) => Dir::add_child(dir, tmp.clone()),
                    None => ()
                }
                dir = Some(tmp);
                // Keep a reference to the root so the parent doesn't get dealocated when we go down the tree xddd
                if root.is_none() {
                    root = dir.clone();
                }
                pwd = None
            }
        }
    };
    println!("{}", Dir::task(root.unwrap()))
}
