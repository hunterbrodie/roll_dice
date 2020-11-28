use std::env;
use rand::Rng;

fn main()
{
    let args: Vec<String> = env::args().collect();

    let split: Vec<i32> = args[1].split('d').map(|x| x.parse::<i32>().unwrap()).collect();

    if split.len() != 2 || args.len() != 2
    {
        println!("Could not understand command");
    }
    else
    {
        let rolls: Vec<i32> = (0..split[0]).map(|_| rand::thread_rng().gen_range(1, split[1] + 1)).collect();

        println!("{:?}", rolls);
    }
}