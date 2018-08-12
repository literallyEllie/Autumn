use std::process;

// Shutdown the app
command!(shutdown(context, _message) {
    context.quit();
    process::exit(1);
});