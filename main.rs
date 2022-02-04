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

                                                                                                       
                                                                                                       
                                                                                                                                                                            
