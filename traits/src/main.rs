trait User {
    // Temos um construtor onde vamos receber um nome de
    // usuário (login)
    fn new(username: &'static str) -> Self;
    // retorna o login definido em new
    fn username(&self) -> &'static str;
    // logar-se no sistema
    fn login(&self) -> &'static str;
    // deslogar-se no sistema
    fn logout(&self) -> &'static str;
    // verificar se está logado
    fn is_logged_in(&self) -> bool {
        false
    }
}
struct Admin {
    username: &'static str,
}
struct Operador {
    username: &'static str,
}
struct BasicUser {
    username: &'static str,
}
impl User for Admin {
    fn new(username: &'static str) -> Admin {
        Admin { username: username }
    }
    fn username(&self) -> &'static str {
        self.username
    }
    fn login(&self) -> &'static str {
        "Usuário do tipo ADMIN entrou no sistema"
    }
    fn logout(&self) -> &'static str {
        "Usuário do tipo ADMIN saiu do sistema"
    }
}
impl User for Operador {
    fn new(username: &'static str) -> Operador {
        Operador { username: username }
    }
    fn username(&self) -> &'static str {
        self.username
    }
    fn login(&self) -> &'static str {
        "Usuário do tipo OPERADOR entrou no sistema"
    }
    fn logout(&self) -> &'static str {
        "Usuário do tipo OPERADOR saiu do sistema"
    }
}
impl User for BasicUser {
    fn new(username: &'static str) -> BasicUser {
        BasicUser { username: username }
    }
    fn username(&self) -> &'static str {
        self.username
    }
    fn login(&self) -> &'static str {
        "Usuário do tipo BÁSICO entrou no sistema"
    }
    fn logout(&self) -> &'static str {
        "Usuário do tipo BÁSICO saiu do sistema"
    }
}
fn main() {
    let admin: Admin = User::new("Corleone");
    println!("Bem-vindo usuário {}", admin.username());
    println!("{}", admin.login());
    println!("{}", admin.logout());
    let operador: Operador = User::new("PessoaQualquer");
    println!("Bem-vindo usuário {}", operador.username());
    println!("{}", operador.login());
    println!("{}", operador.logout());
    let basic_user: BasicUser = User::new("PessoaQualquer2");
    println!("Bem-vindo usuário {}", basic_user.username());
    println!("{}", basic_user.login());
    println!("{}", basic_user.logout());
}
