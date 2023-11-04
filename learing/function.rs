

fn main(){
    let ans: f64 = do_stuff(1.0,3.0);
 println!("{}",ans);
 
 }
 
 fn do_stuff(qty: f64 ,oz :f64) -> f64{
 
     println!("{} {} -oz sarasarperilla(S)",qty,oz);
     qty * oz
     //return qty * oz;
 
 }