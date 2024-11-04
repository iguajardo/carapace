use crate::input_buffer::InputBuffer;

#[repr(i32)]
pub enum PrepareResult {
    Success = 0,
    UnrecognizedStatement = 1,
}

#[repr(i32)]
pub enum StatementType {
    StatementInsert = 0,
    StatementSelect = 1,
}

pub struct Statement {
    pub statement_type: StatementType,
}

pub fn prepare_statement(input: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    match input.buffer.as_str() {
        "insert" => { // string declared like this are stored in stack because they are static
            statement.statement_type = StatementType::StatementInsert;
            PrepareResult::Success
        }
        "select" => {
            statement.statement_type = StatementType::StatementSelect;
            PrepareResult::Success
        }
        _ => {
            println!("failed to recognize given statement");
            PrepareResult::UnrecognizedStatement
        }
    }
}

pub fn execute_statement(statement: &mut Statement) {
    match statement.statement_type {
        StatementType::StatementInsert => {
            println!("This is where we would do a insert.");
        }
        StatementType::StatementSelect => {
            println!("This is where we would do a select.");
        }
    }
}
