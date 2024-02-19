use colored::*;
pub(super) fn print_help() {
    println!("{} !\n\n", "Welcome to Simple DB".green().bold());

    println!(
        "{} - to exit prompt - all db memory changes are flushed to disk before exit. \n\n",
        ".exit".bold().yellow().italic()
    );

    println!("{} \n {}\n\n", 
        "use lowercase for db commands.".bold(),
        "This is to simplify case sensitivity for meta names like tables, columns and values like column value".yellow());

    println!(
        "{} \neg. {} \nlimitation {}\n\n",
        "create".bold().yellow().italic(),
        "create table demo (id int, name text);".green(),
        "current implementation only supports int and text".yellow()
    );

    println!(
        "{} \neg. {} \nlimitation {}\n\n",
        "select".bold().yellow().italic(),
        "select id, name, age from student;".green(),
        "current implementation requires all column and same order as create table".yellow()
    );

    println!(
        "{} \neg. {} \nlimitation {}\n\n",
        "insert".bold().yellow().italic(),
        "insert into student (id, name, age) values (10, harry, 21);".green(),
        "current implementation requires all column and same order as create table".yellow()
    );
}
