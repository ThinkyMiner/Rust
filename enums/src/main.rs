//How are enums different from structures 
//-> enums are used to differentiate between multiple types of a particlar thing
//like in the following example

enum IpAddrImplementation{
    V4,
    V6,
}
//Other way to define it by teliing it the type of input for the fields
enum IpAddrImpl{
    V4(String),// Here for the string part we can just think of what type of input to give in the field
    V6(String),// it can be of any format it can also look like (u8, u8, u8, u8)
}

struct IpAddr{
    kind:IpAddrImplementation,
    address:String,
}

impl IpAddrImplementation{
    fn route(ip : IpAddrImplementation){

    }
}

fn main(){
    let four = IpAddrImplementation::V4; // Enum::Identifier
    let six = IpAddrImplementation::V6;

    let home = IpAddr{
        kind : IpAddrImplementation::V4,
        address : "127.0.0.1".to_string(),
    }

    let hostel = IpAddr{
        kind : IpAddrImpl::V4("127.0.0.3".to_string());
        // for a new struct where we dont require the address seperately
    }

    let office = IpAddr{
        kind : IpAddrImplementation::V6,
        address : "127.0.0.2".to_string(),
    }

}