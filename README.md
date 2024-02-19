# simple_db
A basic toy db written in Rust.
This implementation takes inspiration and add lots of flexibility on top of [this](https://cstack.github.io/db_tutorial/).

### how to run
Follow [rust installation](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you do not have rust installed. <br/>
`cargo run -- -d simple.db` this will create `simple.db` directory where schema and table files are stored. <br/>
You can choose to give different name. <br/>
To make code execution, we are using [tokio tracing](https://tokio.rs/tokio/topics/tracing) with level `info`.

### Sample interaction
![image](./assets/sample_execution.png)

### project goals
|goal                 |status                   |description
|---------------------|-------------------------|------------------
|parse                |:white_check_mark:       |parse create, select, insert commands
|preload              |:white_check_mark:       |load db with student table with sample records.
|persist              |:white_check_mark:       |on exit all changes are flushed to disk which is loaded when simple_db comes back.
|support types        |:white_check_mark:       |int as `i64` and text as `String`
|b tree               |:pencil:                 |save pages in b-tree structure.
|advanced parsing     |:pencil:                 |allow out of order column names.
|projection           |:pencil:                 |for `select` command allow projection.
|primary key          |:pencil:                 |allow unique id by supporting primary key.
|join                 |:pencil:                 |basic join on ids.