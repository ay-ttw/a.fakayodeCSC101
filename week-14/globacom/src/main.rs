use std::io::Read;
use std::io;

fn admin(){

    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn project (){

    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn employee(){

    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn customer(){

    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn vendor(){

    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

}

fn main() {
    let mut input1 =String::new();
    println!("Please state your user class: (Administrator, Project Manager, Employee, Customer or Vendor)");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let class = input1.trim();

    if class == "Administrator"{
    admin()
    }

     if class == "Project Manager"{
        project()
        }

         if class == "Employee"{
            employee()
            }
             if class == "Customer"{
                customer()
                }

                 if class == "Vendor"{
                    vendor()
                    }
    

}
