use std::io;
use rand::Rng;
//use std::cmp::Ordering;


fn main(){

    

    let secret_num = rand::thread_rng().gen_range(1..10);
    loop{

        println!("ENTER A NUMBER TO GUESS:- ");
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)    
        .expect("Error");

        println!("You guessed {}", guess);

        

        let guess:u32=guess.trim().parse().expect("Please Type A Number.");
       // let secret_num=secret_num as f32;

        //println!("Factorising the number {}", guess);
        let mut number=1;

        while number<=guess{
           if guess%number==0{
                println!("{}",number);
            }
            number+=1;
        }
        
        //if guess<secret_num{
          //  println!("Too Small");
        //}
        //else if guess>secret_num{
         //   println!("Too High");
        //}
        //else if guess==secret_num{
          //  println!("You win");
            //break;
        //}

        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal  => {
                println!("You Win!!");
                break;
            }
        }
    }

}