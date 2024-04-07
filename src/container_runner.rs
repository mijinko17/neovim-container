use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{cli::Args, path::PathUtils};

pub fn run_container(args: Args<PathBuf>, dir_state_provider: impl DirectoryStateProvider) {
    let cors: Vec<Box<dyn CreateNvimCommandExecutorCor>> = vec![
        Box::new(DirectoryCor {
            dir_state_provider: &dir_state_provider,
        }),
        Box::new(DirUnderHomeDirCor {
            dir_state_provider: &dir_state_provider,
        }),
        Box::new(DirNotUnderHomeDirCor {
            dir_state_provider: &dir_state_provider,
        }),
    ];
    if let Some(executor) = cors.into_iter().find_map(|cor| cor.create(args.clone())) {
        executor.execute()
    }
}

struct VolumeArg {
    host_path: PathBuf,
    container_path: PathBuf,
}

impl VolumeArg {
    pub fn new(host_path: impl AsRef<Path>, container_path: impl AsRef<Path>) -> VolumeArg {
        VolumeArg {
            host_path: host_path.as_ref().to_path_buf(),
            container_path: container_path.as_ref().to_path_buf(),
        }
    }
    pub fn raw_volume_arg(self) -> Vec<String> {
        vec![
            "--volume".to_string(),
            format!(
                "{}:{}",
                self.host_path.to_str().unwrap(),
                self.container_path.to_str().unwrap()
            ),
        ]
    }
}

struct NvimCommandExecutor<T: AsRef<Path>, U: AsRef<Path>> {
    image: &'static str,
    volumes: Vec<VolumeArg>,
    work_dir: T,
    target_file_path: Option<U>,
}

impl<T, U> NvimCommandExecutor<T, U>
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    pub fn execute(self) {
        Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("--interactive")
            .arg("--tty")
            .args(vec!["--workdir", self.work_dir.as_ref().to_str().unwrap()])
            .arg("--network=host")
            .args(
                self.volumes
                    .into_iter()
                    .flat_map(|arg| arg.raw_volume_arg())
                    .collect::<Vec<_>>(),
            )
            .arg(self.image)
            .arg("nvim")
            .optional_arg(
                self.target_file_path
                    .and_then(move |p| p.as_ref().to_str().map(|s| s.to_string())),
            )
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

pub trait DirectoryStateProvider {
    fn current_dir(&self) -> Option<PathBuf>;
    fn home_dir(&self) -> Option<PathBuf>;
    fn absolute_path(&self, relative_path: impl AsRef<Path>) -> PathBuf;
}

pub trait OptionalArg {
    fn optional_arg<S: AsRef<OsStr>>(&mut self, optional_arg: Option<S>) -> &mut Self;
}

impl OptionalArg for Command {
    fn optional_arg<S: AsRef<OsStr>>(&mut self, optional_arg: Option<S>) -> &mut Self {
        match optional_arg {
            Some(value) => self.arg(value),
            None => self,
        }
    }
}

trait CreateNvimCommandExecutorCor {
    // fn is_responsible(&self, args: Args<impl AsRef<Path>>) -> bool;
    fn create(&self, args: Args<PathBuf>) -> Option<NvimCommandExecutor<PathBuf, PathBuf>>;
}

struct DirUnderHomeDirCor<'a, T: DirectoryStateProvider> {
    dir_state_provider: &'a T,
}

impl<'a, T> CreateNvimCommandExecutorCor for DirUnderHomeDirCor<'a, T>
where
    T: DirectoryStateProvider,
{
    fn create(&self, _args: Args<PathBuf>) -> Option<NvimCommandExecutor<PathBuf, PathBuf>> {
        let home_dir = self.dir_state_provider.home_dir()?;
        let current_dir = self.dir_state_provider.current_dir()?;
        let work_dir = Path::new("/home/host")
            .join(current_dir.relative_path_from_ancsestor(home_dir.clone())?);
        Some(NvimCommandExecutor {
            image: "mijinko17/neovim-container:latest",
            volumes: vec![
                VolumeArg::new(home_dir.clone(), Path::new("/home/host")),
                VolumeArg::new(
                    home_dir.clone().join(Path::new(".gitconfig")),
                    Path::new("/home/neovim/.gitconfig"),
                ),
                VolumeArg::new(
                    home_dir.join(Path::new(".ssh")),
                    Path::new("/home/neovim/.ssh"),
                ),
            ],
            work_dir,
            target_file_path: None as Option<PathBuf>,
        })
    }
}

struct DirNotUnderHomeDirCor<'a, T: DirectoryStateProvider> {
    dir_state_provider: &'a T,
}

impl<'a, T> CreateNvimCommandExecutorCor for DirNotUnderHomeDirCor<'a, T>
where
    T: DirectoryStateProvider,
{
    fn create(&self, _args: Args<PathBuf>) -> Option<NvimCommandExecutor<PathBuf, PathBuf>> {
        let home_dir = self.dir_state_provider.home_dir()?;
        let current_dir = self.dir_state_provider.current_dir()?;
        println!("{:?}", current_dir);
        if home_dir.is_ancestor_of(&current_dir) {
            None
        } else {
            let work_dir = Path::new("/home/host").to_path_buf();
            Some(NvimCommandExecutor {
                image: "mijinko17/neovim-container:latest",
                volumes: vec![VolumeArg::new(current_dir, Path::new("/home/host"))],
                work_dir,
                target_file_path: None as Option<PathBuf>,
            })
        }
    }
}

struct DirectoryCor<'a, T: DirectoryStateProvider> {
    dir_state_provider: &'a T,
}

impl<'a, T> CreateNvimCommandExecutorCor for DirectoryCor<'a, T>
where
    T: DirectoryStateProvider,
{
    fn create(&self, _args: Args<PathBuf>) -> Option<NvimCommandExecutor<PathBuf, PathBuf>> {
        let home_dir = self.dir_state_provider.home_dir()?;
        let current_dir = self.dir_state_provider.current_dir()?;
        let work_dir = Path::new("/home/host").to_path_buf();
        Some(NvimCommandExecutor {
            image: "mijinko17/neovim-container:latest",
            volumes: vec![
                VolumeArg::new(current_dir, Path::new("/home/host")),
                VolumeArg::new(
                    home_dir.clone().join(Path::new(".gitconfig")),
                    Path::new("/home/neovim/.gitconfig"),
                ),
                VolumeArg::new(
                    home_dir.join(Path::new(".ssh")),
                    Path::new("/home/neovim/.ssh"),
                ),
            ],
            work_dir,
            target_file_path: None as Option<PathBuf>,
        })
    }
}

struct FileCor<'a, T: DirectoryStateProvider> {
    dir_state_provider: &'a T,
}

impl<'a, T> CreateNvimCommandExecutorCor for FileCor<'a, T>
where
    T: DirectoryStateProvider,
{
    fn create(&self, args: Args<PathBuf>) -> Option<NvimCommandExecutor<PathBuf, PathBuf>> {
        let target = args.path?;
        let home_dir = self.dir_state_provider.home_dir()?;
        let current_dir = self.dir_state_provider.current_dir()?;
        let work_dir = Path::new("/home/host").to_path_buf();
        Some(NvimCommandExecutor {
            image: "mijinko17/neovim-container:latest",
            volumes: vec![
                VolumeArg::new(current_dir, Path::new("/home/host")),
                VolumeArg::new(
                    home_dir.clone().join(Path::new(".gitconfig")),
                    Path::new("/home/neovim/.gitconfig"),
                ),
                VolumeArg::new(
                    home_dir.join(Path::new(".ssh")),
                    Path::new("/home/neovim/.ssh"),
                ),
            ],
            work_dir,
            target_file_path: None as Option<PathBuf>,
        })
    }
}
