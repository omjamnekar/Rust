


fn main(){


    // tuples
    
    let info =(1,3.3,999);
    
    // let info:(u128,f64,i128) =(1,3.3,999);
    
    // "_" keeping variable private
    // let _jets= info.0;
    
    // let _fuel= info.1;
    
    // let _ammo= info.2;
    
    let(_jets,_fuel,_ammo) =info; 
     println!("{} {} {}",_jets,_ammo,_fuel);
    
    
    //array
    
    let buf:[u8;3]  =[1,2,3];
    
    //arrays are limited to the 32 
    // 32<loss
    //thats why we use "Vectores"
    
    }