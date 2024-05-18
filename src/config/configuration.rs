use std::env;

/// Este método foi criado para ler as configurações de variável de ambiente <br>
/// Utilizamos o arquivo env.sh para carregar as variáveis, renomeie o arquivo <b>env.sh.sample</b> para <b>env.sh</b> assim vc terá todas as variáveis de ambiente
///
/// # Conteúdo interno do env.sh.sample
///
/// ```
/// export DATABASE_USER="root"
/// export DATABASE_PASSWORD="root"
/// export DATABASE_DB="desafio_rust_alunos_com_orm"
/// export DATABASE_HOST="localhost"
///  ```
///
/// # Retorno
///
/// Retorna uma string de conexão assim:
/// ```
/// "mysql://root:root@localhost/database"
/// ```
pub fn get_mysql_string_connection() -> String {
    let user:String = match env::var("DATABASE_USER") {
        Ok(value) => value,
        Err(_) => "root".to_string(),
    };

    let pass:String = match env::var("DATABASE_PASSWORD") {
        Ok(value) => value,
        Err(_) => "".to_string(),
    };

    let db:String = match env::var("DATABASE_DB") {
        Ok(value) => value,
        Err(_) => "seu_db".to_string(),
    };

    let host:String = match env::var("DATABASE_HOST") {
        Ok(value) => value,
        Err(_) => "localhost".to_string(),
    };

    format!("mysql://{}:{}@{}/{}", user, pass, host, db)
}