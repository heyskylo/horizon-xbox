//! Horizon on Xbox One: the UWP shell an Xbox in Dev Mode runs (packaging/xbox). It opens a CoreWindow,
//! runs a check of the simulation core on the console, and writes the answer to the app's LocalState
//! folder (`horizon-xbox.log`, reachable from Dev Home's file browser). The game itself still needs a
//! UWP window and input layer for winit's place; see packaging/xbox/README.md.
#![windows_subsystem = "windows"]

/// What the console says the simulation core computes: Java's `Random` and HotSpot-exact `Math`.
fn probe() -> String {
    use horizon_core::random::RandomSource;
    let mut random = horizon_core::random::LegacyRandom::new(42);
    format!(
        "Horizon on Xbox: Java Random(42).nextInt(100) = {:?}, sin(1.0) = {}",
        random.next_int_bound(100),
        horizon_core::jmath::sin(1.0)
    )
}

#[cfg(windows)]
mod uwp {
    use windows::{
        ApplicationModel::Core::{
            CoreApplication, CoreApplicationView, IFrameworkView, IFrameworkViewSource,
            IFrameworkViewSource_Impl, IFrameworkView_Impl,
        },
        Storage::ApplicationData,
        UI::Core::{CoreProcessEventsOption, CoreWindow},
        Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
    };
    use windows_core::{implement, Result, HSTRING};

    #[implement(IFrameworkViewSource)]
    struct Source;

    impl IFrameworkViewSource_Impl for Source_Impl {
        fn CreateView(&self) -> Result<IFrameworkView> {
            Ok(View.into())
        }
    }

    #[implement(IFrameworkView)]
    struct View;

    impl IFrameworkView_Impl for View_Impl {
        fn Initialize(&self, _: Option<&CoreApplicationView>) -> Result<()> {
            Ok(())
        }
        fn SetWindow(&self, _: Option<&CoreWindow>) -> Result<()> {
            Ok(())
        }
        fn Load(&self, _: &HSTRING) -> Result<()> {
            Ok(())
        }
        fn Run(&self) -> Result<()> {
            let folder = ApplicationData::Current()?.LocalFolder()?.Path()?;
            let _ = std::fs::write(
                std::path::Path::new(&folder.to_string_lossy()).join("horizon-xbox.log"),
                super::probe(),
            );
            let window = CoreWindow::GetForCurrentThread()?;
            window.Activate()?;
            window
                .Dispatcher()?
                .ProcessEvents(CoreProcessEventsOption::ProcessUntilQuit)
        }
        fn Uninitialize(&self) -> Result<()> {
            Ok(())
        }
    }

    pub fn run() -> Result<()> {
        // SAFETY: once, on the app's own thread, before any WinRT call.
        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok()? };
        CoreApplication::Run(&IFrameworkViewSource::from(Source))
    }
}

fn main() {
    #[cfg(windows)]
    if let Err(e) = uwp::run() {
        eprintln!("Horizon: {e}");
    }
    #[cfg(not(windows))]
    println!("{}", probe());
}
