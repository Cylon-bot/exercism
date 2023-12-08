use time::PrimitiveDateTime as DateTime; 
use time::Duration; 

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime { 
    let duration_of_gigasecond: Duration = Duration::seconds(1000000000); 
    start + duration_of_gigasecond 
}