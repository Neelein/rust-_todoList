use std::io::{self, Stdout};

use::rust_command_todo_list::todo_model::Todo;
use::rust_command_todo_list::list::MyLsit;
use chrono::prelude::*;
use tui::widgets::{List,ListItem,Block,Borders};
use tui::style::{Color,Style,Modifier};
use tui::{backend::CrosstermBackend};
use tui::Terminal;



fn main() -> Result<(),io::Error> {

    let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
    let list = List::new(items)
    .block(Block::default().title("List").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    .highlight_symbol(">>");

    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    // let mut todo_list = Vec::new(); 
    // let mylist = MyLsit{data:todo_list};
    // mylist.add_to_list();
    
    // for _n in 1..10{
    //     let todo = Todo {date_time:Utc::now(),name:"test".to_string(),id:1,state:"normal".to_string()};
    //     todo_list.push(todo);
    // }


    println!("{:?}",list);

    Ok(())
}
