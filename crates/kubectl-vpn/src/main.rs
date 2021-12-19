#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(vpn =>
        (version: "0.0.1")
        (author: "Jakub Al-Khalili <me@alkhalili.pl>")
        (about: "CLI for KubeVPN")
        (@subcommand connect =>
            (about: "connect to current Kubernetes cluster")
        )
    ).get_matches();
}
