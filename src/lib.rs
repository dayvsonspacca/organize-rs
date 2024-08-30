use std::collections::HashMap;

#[derive(Debug)]
pub enum OrganizeMethod {
    // Organizar por extensão de arquivo, pordendo configura as pastas e extensões.
    Extension(HashMap<&'static str, Vec<&'static str>>),

    // Organizar por data de atualização do arquivo.
    Date,

    // Organizar por inicial do arquivo.
    Alphabetical,

    // Organizar por tamanho do arquivo.
    Size,
}

impl OrganizeMethod {
    pub fn custom_extension(extensions: HashMap<&'static str, Vec<&'static str>>) -> Self {
        OrganizeMethod::Extension(extensions)
    }
}

#[derive(Debug)]
pub struct OrganizeOptions {
    method: OrganizeMethod,

    // Determinar se a lógica de ordenação vai ser feita parelalmente em varios processos.
    // Cada processo vai organizar certos arquivos.
    // e.g:
    // Para o método de organização por `Extension` será levantado um processo para organizar cada tipo de arquivo, Documentos, Imagens e etc
    parallel: bool,
}

impl OrganizeOptions {
    pub fn new(method: OrganizeMethod, parallel: bool) -> Self {
        Self { method, parallel }
    }

    /// Configuração padrão para organizar pastas, vai organizar por `OrganizeMethod::Extension`
    /// e vai criar um proceso para cada pasta a se organizar com `std::thread::spawn()`
    ///
    /// Onde vai separar os arquivos nas pastas.
    /// - Documents -> `[".pdf", ".txt", ".docs"]`
    /// - Images -> `[".png", ".jpg", ".jpeg"]`
    /// - Audios -> `[".mp3", ".wav", ".flac"]`
    /// - Videos -> `[".mp4", ".mov", ".avi"]`
    /// - Sheets -> `[".csv", ".xlsx", ".ods"]`
    pub fn default() -> Self {
        let mut default_extensions: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

        default_extensions.insert("Documents", vec![".pdf", ".txt", ".docs"]);
        default_extensions.insert("Images", vec![".png", ".jpg", ".jpeg"]);
        default_extensions.insert("Audios", vec![".mp3", ".wav", ".flac"]);
        default_extensions.insert("Videos", vec![".mp4", ".mov", ".avi"]);
        default_extensions.insert("Sheets", vec![".csv", ".xlsx", ".ods"]);

        Self {
            method: OrganizeMethod::Extension(default_extensions),
            parallel: true,
        }
    }

    /// Especifica o método de organização.
    pub fn set_method(&mut self, method: OrganizeMethod) {
        self.method = method;
    }

    /// Especifica se o método de organização deve rodar em paralelo quando possível ou não.
    pub fn set_parallel(&mut self, parallel: bool) {
        self.parallel = parallel;
    }
}
