use std::collections::{BTreeSet, HashMap};

fn main() {

   let input_data=["пятак","пятка","тяпка","листок","слиток","столик","Пятка","mimo"].to_vec();
   let anagramms=get_anagramms(&input_data);
   for (key,anagramms) in anagramms  {

    println!("Key: {},anagramms:{:?}",key,anagramms)
       
   }
}


type Anagramms=HashMap<String,BTreeSet<String>>;


fn get_anagramms(data:&[&str])->Anagramms{


    let mut vault=HashMap::new();
    data.into_iter().for_each(|anagramm|{
     let anagramm=anagramm.to_lowercase();
     let mut chars=anagramm.chars().collect::<Vec<char>>();
     chars.sort();
     let key_anagramm=chars.into_iter().collect::<String>();
     vault.entry(key_anagramm).or_insert(BTreeSet::new()).insert(anagramm);
    });
    vault.into_iter().filter(|(_,annagrams)|{
             annagrams.len()>1
    }).collect()


}