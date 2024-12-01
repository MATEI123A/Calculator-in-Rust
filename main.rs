
fn number(temp: &str) -> (f32, usize) {
    let mut nr = 0f32;
    let mut tempo:String=String::new();
    let mut ok=0;
    let mut l = 0usize;

    for c in temp.chars() {
        if c.is_digit(10) {
            tempo.push(c);
            l += 1;
        } else if c=='.' {
            ok=1;
            tempo.push(c);
        } else {
            break;
        }
    }

    if ok==0{
        tempo.push('.');
        tempo.push('0');
    }

    nr=tempo.parse::<f32>().unwrap();
    (nr, l)
}

fn is_point(temp:&str)-> bool{
    for c in temp.chars() {
        if c=='.'{
            return true;
        } else if !c.is_digit(10) {
            return false;
        }
    }

    return false;
}

fn main()->Result<(),String>{
    let operation = String::from("1+2*32");
    let mut s: f32 = 0f32;
    let mut ok=0;
    let mut index = 0;

    if operation.chars().nth(index).unwrap() == '-'
    {
        index += 1;
        let temp = &operation[index..];

        if is_point(temp)==true{
            ok=1;
        }

        let (nr, l) = number(temp);
        index+=l;
        s-=nr;
    }

    while index < operation.len() {
        let temp = &operation[index..];

        if is_point(temp)==true{
            ok=1;
        }

        let (nr, l) = number(temp);

        if s == 0f32 && index == 0 {
            s += nr;
        }

        index += l;

        if index < operation.len() {
            match operation.chars().nth(index).unwrap() {
                '+' => {
                    index += 1;
                    let temp = &operation[index..];

                    if is_point(temp)==true{
                        ok=1;
                    }

                    let (next_nr, l) = number(temp);
                    let mut nr=next_nr;
                    index += l;

                    if index < operation.len() && operation.chars().nth(index).unwrap() == '^' {
                        index+=1;
                        let temp = &operation[index..];

                        if is_point(temp)==true{
                            ok=1;
                        }

                        let (next_nr, l) = number(temp);
                        index+=l;

                        let mut copie=next_nr-1f32;

                        while copie>0f32{
                            nr*=nr;
                            copie-=1f32;
                        }
                    }

                    while  index < operation.len() && operation.chars().nth(index).unwrap() == '*'  {
                        index+=1;
                        let temp = &operation[index..];

                        if is_point(temp)==true{
                            ok=1;
                        }

                        let (next_nr, l) = number(temp);
                        index+=l;
                        nr*=next_nr;
                    }

                    while  index < operation.len() && operation.chars().nth(index).unwrap() == '/'  {
                        index+=1;
                        let temp = &operation[index..];

                        if is_point(temp)==true{
                            ok=1;
                        }

                        let (next_nr, l) = number(temp);

                        if next_nr==0f32 {
                            return Err("Can not divide by 0".to_string());
                        }

                        index+=l;
                        nr/=next_nr;
                    }

                    s+=nr;

                    if index == operation.len() {
                        break;
                    }
                }
                '-' => {
                    index += 1;
                    let temp = &operation[index..];

                    if is_point(temp)==true{
                        ok=1;
                    }

                    let (next_nr, l) = number(temp);
                    let mut nr=next_nr;
                    index += l;

                    if index < operation.len() && operation.chars().nth(index).unwrap() == '^' {
                        index+=1;
                        let temp = &operation[index..];

                        if is_point(temp)==true{
                            ok=1;
                        }

                        let (next_nr, l) = number(temp);
                        index+=l;

                        let mut copie=next_nr-1f32;

                        while copie>0f32{
                            nr*=nr;
                            copie-=1f32;
                        }
                    }

                    while  index < operation.len() && operation.chars().nth(index).unwrap() == '*'  {
                        index+=1;
                        let temp = &operation[index..];

                        if is_point(temp){
                            ok=1;
                        }

                        let (next_nr, l) = number(temp);
                        index+=l;
                        nr*=next_nr;
                    }

                    while  index < operation.len() && operation.chars().nth(index).unwrap() == '/'  {
                        index+=1;
                        let temp = &operation[index..];

                        if is_point(temp){
                            ok=1;
                        }

                        let (next_nr, l) = number(temp);

                        if next_nr==0f32 {
                            return Err("Can not divide by 0".to_string());
                        }

                        index+=l;
                        nr/=next_nr;
                    }

                    s-=nr;

                    if index == operation.len() {
                        break;
                    }
                }

                '*' =>{
                    index += 1;
                    let temp = &operation[index..];

                    if is_point(temp){
                        ok=1;
                    }

                    let (next_nr, l) = number(temp);
                    index += l;
                    let mut nr=next_nr;

                    if index < operation.len() && operation.chars().nth(index).unwrap() == '^' {
                        index+=1;
                        let temp = &operation[index..];

                        if is_point(temp){
                            ok=1;
                        }

                        let (next_nr, l) = number(temp);
                        index+=l;

                        let mut copie=next_nr-1f32;

                        while copie>0f32{
                            nr*=nr;
                            copie-=1f32;
                        }
                    }

                    s *= nr;
                }

                '/' =>{
                    index += 1;
                    let temp = &operation[index..];

                    if is_point(temp){
                        ok=1;
                    }

                    let (next_nr, l) = number(temp);

                    if next_nr==0f32 {
                        return Err("Can not divide by 0".to_string());
                    }

                    s /= next_nr;
                    index += l;
                }

                '^'=>{
                    index+=1;

                    let temp = &operation[index..];

                    if is_point(temp){
                        ok=1;
                    }

                    let (next_nr, l) = number(temp);
                    let mut n=next_nr-1f32;
                    let copie=s;

                    while n>0f32{
                        s *= copie;
                        n-=1f32;
                    }

                    index += l;
                }
                _ => {}
            }
        }
    }

    if ok==0 {
        println!("Rezultatul este : {}", s as i32);
    }
    else {
        println!("Rezultatul este: {}",s);
    }

    Ok(())
}
