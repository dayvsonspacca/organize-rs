use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{copy, create_dir_all, read_dir},
    path::Path,
};

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

    fn execute(self, _parallel: bool, path: &'static str) {
        let files = self.get_files(path);

        if let OrganizeMethod::Extension(extensions) = self {
            for file_name in files {
                if let Some(extension) = file_name.split('.').last() {
                    for (folder_name, ext_list) in &extensions {
                        if ext_list.contains(&extension) {
                            let folder_path = Path::new(path).join(folder_name);

                            if let Err(e) = create_dir_all(&folder_path) {
                                eprintln!(
                                    "Erro ao criar diretório {}: {}",
                                    folder_path.display(),
                                    e
                                );
                            }

                            let file_path = Path::new(path).join(&file_name);
                            if !file_path.exists() {
                                eprintln!("Arquivo não encontrado: {}", file_path.display());
                                continue;
                            }

                            let dest_path =
                                folder_path.join(file_path.file_name().unwrap_or(OsStr::new("")));

                            if let Err(e) = copy(file_path, &dest_path) {
                                eprintln!(
                                    "Erro ao copiar arquivo {} para {}: {}",
                                    file_name,
                                    dest_path.display(),
                                    e
                                );
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    fn get_files(&self, path: &'static str) -> Vec<String> {
        if let OrganizeMethod::Extension(_) = self {
            let mut file_names = Vec::new();
            if let Ok(entries) = read_dir(path) {
                for entry in entries.flatten() {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        if let Some(file_name) = file_path.file_name().and_then(|f| f.to_str()) {
                            file_names.push(file_name.to_string());
                        }
                    }
                }
            }
            file_names
        } else {
            Vec::new()
        }
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

        default_extensions.insert("Documents", vec!["pdf", "txt", "docs"]);
        default_extensions.insert("Images", vec!["png", "jpg", "jpeg"]);
        default_extensions.insert("Audios", vec!["mp3", "wav", "flac"]);
        default_extensions.insert("Videos", vec!["mp4", "mov", "avi"]);
        default_extensions.insert("Sheets", vec!["csv", "xlsx", "ods"]);

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

pub struct Organize {
    options: OrganizeOptions,
    path: &'static str,
}

impl Organize {
    pub fn new(path: &'static str, options: OrganizeOptions) -> Self {
        Self { path, options }
    }

    pub fn set_path(&mut self, path: &'static str) {
        self.path = path;
    }

    pub fn set_options(&mut self, options: OrganizeOptions) {
        self.options = options;
    }

    pub fn execute(self) -> Result<(), std::io::Error> {
        if self.path.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Path cannot be empty",
            ));
        }

        self.options
            .method
            .execute(self.options.parallel, self.path);

        Ok(())
    }
}
