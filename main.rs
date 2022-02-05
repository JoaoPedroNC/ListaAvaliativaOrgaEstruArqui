use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::mem;
use std::path::Path;

#[derive(Copy, Clone)]
struct RegNascimento{
                     cod_municipio_nasci: [u8;6],/* A variável em questão representa o Código do Município de Nascimento*/
                     cod_estabelecimento:[u8;7],/*Esta variável representa o código do estabelecimento*/
                     cod_municipio_resi:[u8;6],/*A variável em questão representa o Código do Municípo de Residência*/
                     data_nasc:[u8;8],/*Se trata da Data de Nascimento no formato DDMMAAAA*/
                     semanas_gestacao:[u8;2],/*Esta variável representa o Número de Semanas de Gestação*/
                     sexo:[u8;1],/*Sexo 0 para não informado, 1 para masculino ou 2 para feminino*/
                     peso:[u8;4],/*Se trata do peso em gramas*/
                     data_nasci_mae:[u8;8],/*Data de Nascimento da mãe do indivíduo no formato DDMMAAAA*/
                     }//6+7+6+8+2+1+4+8=42 bytes que serão armazenados
                     
//O código a seguir converte uma array de u8 para String
fn u8_to_string(s:&[u8]) ->String{
                                  s.iter().map(|&c| c as char).collect()
                                  }

//1)Qual é o tamanho do arquivo em bytes?
fn questao_1(arquivo_original: &File) -> u64{
                                             match arquivo_original.metadata(){
                                                                               Ok(metadata) => metadata.len(),
                                                                               Err(e) =>panic!("Erro ao ler o tamanho do arquivo: {}",e),
                                                                               }
                                             }
                                             
/*4) Copie em um novo arquico chamado "sinasc-sp-capital-2018.dat" os registros dos nascimentos(CODMUNNASC)que ocorreream na capital,
cujo código é "355030".Quantos registros tem esse novo arquivo?*/
fn questao_4(arquivo_original: &mut File, num_registros: u64){
                                                              let mut arquivo_copia = match File::create(Path::new("sinasc-sp-capital-2018.dat")){
                                                                                                                                                  Ok(file) => file,
                                                                                                                                                  Err(e) => Panic!("Erro ao criar o arquivo:{}",e),
                                                                                                                                                  };
                                                              let mut buffer = BufReader:: new(arquivo_original);
                                                              buffer.seek(SeekFrom::Start(0)).unwrap();
                                                              let mut contagem_nasci_capital = 0;
                                                              
                                                              let mut registro = [0;42];
                                                              for _ in 0..num_registros{
                                                                                        match buffer.read(&mut registro){
                                                                                                                         Ok(size)=>{
                                                                                                                                    if size==0{
                                                                                                                                               break;
                                                                                                                                               }//EOF
                                                                                                                                     let ultimo_reg: RegNascimento = unsafe{ mem:: transmute(registro)};
                                                                                                                                     
                                                                                                                                     if u8_to_string(&ultimo_reg.cod_municipio_nasci)== "355030"{
                                                                                                                                                                                                 contagem_nasci_capital+=1;
                                                                                                                                                                                                 arquivo_copia.write(&registro).unwrap();
                                                                                                                                                                                                 }
                                                                                                                                                }
                                                                                                                                     Err(e) => panic!("Erro ao ler o arquivo:{}",e),
                                                                                                                                     }
                                                                                                                         }
                                                                                                                         println!("Quantidade de registros na capital copiados para \"sinasc_sp-capital-2018.dat\":{}", contagem_nasci_capital)
                                                                                        }
                                                              /* 5)Quantas meninas nasceram em Santos(354850) no ano de 2018? */
                                                              fn questao_5(arquivo_original: &mut File){
                                                                                                        let mut contagem=0;
                                                                                                        let mut buffer= BufReader::new(arquivo_original);
                                                                                                        buffer.seek(SeekFrom::Start(0)).unwrap();
                                                                                                        loop{
                                                                                                             let mut registro=[0;42];
                                                                                                             match buffer.read(&mut registro){
                                                                                                             Ok(size)=>{
                                                                                                             if size==0{
                                                                                                             break;
                                                                                                             }//EOF
                                                                                                             
                                                                                                        let ultimo_reg: RegNascimento= unsafe{mem::transmute(registro)};
                                                                                                        
                                                                                                        if u8_to_string(&ultimo_reg.cod_municipio_nasci)=="354850" && u8_to_string(&ultimo_reg.sexo)=="2"{
                                                                                                                                                                                                          contagem+=1;
                                                                                                                                                                                                          }
                                                                                                        }
                                                                                                        Err(e)=>panic!("Erro ao ler o arquivo:{}",e),
                                                                                                        }
                                                                                                       }
                                                                                                       
                                                                                                       println!("Quantidade de meninas que nasceram em Santos(354850) no ano de 2018:{}", contagem);
                                                                                                       }
/*6)Quantos bebês nasceram com baixo peso(<2500)em campinas(350950)no ano de 2018?*/
fn questao_6(arquivo_original: &mut File){
                                          let mut contagem=0;
                                          let mut buffer=BufReader::new(arquivo_original);
                                          buffer.seek(SeekFrom::Start(0)).unwrap();
                                          loop{
                                               let mut registro=[0;42];
                                               match buffer.read(&mut registro){
                                                                                Ok(size)=>{
                                                                                           if size==0{
                                                                                                      break;
                                                                                                      }//EOF
                                                                                  
                                                                                  let ultimo_reg: RegNascimento=unsafe{mem::transmute(registro)};
                                                                                  if u8_to_string(&ultimo_reg.cod_municipio_nasci)=="350950"{
                                                                                    let peso:u32=u8_to_string(&ultimo_reg.peso).parse().unwrap();
                                                                                    if peso<2500{
                                                                                      contagem+=1;
                                                                                    }
                                                                                  }
                                                 }
                                                 Err(e)=>panic!("Erro ao ler o registro:{}",e),
                                            }
  }
  println!("Quantidade de bebês que nasceram com baixo peso(<2500) em campinas(350950) no ano de 2018:{}",contagem);
}
/*7)Ordene o arquivo pelo código do estabelecimento, gere o arquivo"sinasc_sp_2018_ordenado.dat".Não é para fazer ordenação externa.*/
fn questao_7(arquivo_original:&mut File){
  let mut arr: Vec<RegNascimento> = Vec::new();
  let mut buffer=BufReader::new(arquivo_original);
  buffer.seek(SeekFrom::Start(0)).unwrap();
  
  let mut registro=[0;42];
  for _ in 0..606146{
    match buffer.read(&mut registro){
      Ok(size)=>{
        if size==0{
          break;
        }//EOF
        let ultimo_reg:RegNascimento=unsafe{mem::transmute(registro)};
        arr.push(ultimo_reg);
      }
      Err(e) => panic!("Erro ao ler o arquivo:{}",e),
    }
  }
  //Ordenando o arquivo
  arr.sort_by(|a,b| a.cod_estabelecimento.cmp(&b.cod_estabelecimento));
  println!("Ordenando o arquivo pela ordem do estabelecimento");
  
  let mut arquivo_copia= match File::create(Path::new("sinasc_sp_2018_ordenado.dat")){
    Ok(file)=>file,
    Err(e) => panic!("Erro ao criar o arquivo:{}",e),
  };
  
  //Gravando dado no arquivo
  for reg in arr.iter(){
    let regitro:[u8:42]= unsafe{mem::transmute(*reg)};
    arquivo_copia.write(&registro).unwrap();
  }
  println!("Arquivo ordenado e gravado em \"sinasc-sp-2018-ordenado.dat\"")
}

/* 8)Com o arquivo ordenado, conte o número de nascimentos por estabelecimento.
   Leia o primeiro registro e atribua ao contador
   1. Enquanto não for final do arquivo, leia os registros subsequentes sempre guardando o código do estabelecimento do registro anterior.
   Quando o estabelecimento mudar ou quando o final do arquivo for alcançado, imprima o contador. 
   Se o registro lido tiver o mesmo código do estabelecimento do anterior, apens acrescente 1 unidade ao contador, sem imprimir.*/
fn questao_8(){
  let mut arquivo_ordenado= match File::open("sinasc-sp-2018-ordenado.dat"){
    Ok(file) => file,
    Err(e) => panic!("Erro ao abrir o arquivo:{}",e),
  };
  
  let mut buffer = BufReader::new(&mut arquivo_ordenado);
  let mut contador=0;
  let mut ultimo_estabelecimento = String::new();
  let mut registro = [0;42];
  for _ in 0..606146{
    match buffer.read(&mut registro){
      Ok(size) =>{
        if size==0{
          break;
        }//EOF
        let reg_atual:RegNascimento= unsafe{mem::transmute(registro)};
        let cod_estabelecimento

                                                 
                                                 
                                                                                                       
                                                                                                       
                                                                                                                                                                            
