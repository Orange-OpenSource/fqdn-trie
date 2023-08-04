#![cfg(feature= "graphviz")]

use crate::{HasFqdn, ALPHABET};
use crate::trie::InnerTrie;
use std::process::{Command, Stdio};
use std::io;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::trie::index::LeafIndex;


static DOTCMD:&str = "dot";

impl<T:HasFqdn> InnerTrie<T>
{
    pub fn generate_pdf_file(&self, file: Option<&str>) -> io::Result<()>
    {
        let child = match file {
            None => {
                Command::new(DOTCMD)
                    .arg("-Tpdf")
                    .stdin(Stdio::piped())
                    .spawn()
            }
            Some(filename) => {
                let mut path = PathBuf::from(filename);
                path.set_extension("pdf");
                eprintln!("write output in file: {}", path.display());

                Command::new(DOTCMD)
                    .arg("-Tpdf")
                    .arg("-o").arg(path)
                    .stdin(Stdio::piped())
                    .spawn()
            }
        };
        let child = match child {
            Err(why) => panic!("couldn't spawn dot: {}", why),
            Ok(process) => process,
        };
        let mut dot = child.stdin.unwrap();
        self.write_dot(&mut dot)
    }

    pub fn generate_graphviz_file(&self, file: Option<&str>) -> io::Result<()>
    {
        match file {
            None => {
                let mut dot = io::stdout();
                self.write_dot(&mut dot)
            }
            Some(filename) => {
                let mut path = PathBuf::from(filename);
                path.set_extension("gv");
                eprintln!("write output in file: {}", path.to_string_lossy());
                let mut dot = File::create(path)?;
                self.write_dot(&mut dot)
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub fn open_dot_view(&self) -> io::Result<()>
    {
        use std::os::unix::io::AsRawFd;
        use std::os::unix::io::FromRawFd;

        let dot = match Command::new(DOTCMD)
            .arg("-Tpdf")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spawn dot: {}", why),
            Ok(process) => process,
        };
        // sur mac seulement...
        unsafe {
            Command::new("open")
                .arg("-f")
                .arg("-a").arg("Preview")
                .stdin(Stdio::from_raw_fd(dot.stdout.unwrap().as_raw_fd()))
                .spawn()?;
        }
        let mut dot = dot.stdin.unwrap();
        self.write_dot(&mut dot)
    }

    fn write_dot(&self, dot: &mut dyn Write) -> io::Result<()>
    {
        writeln!(dot, "digraph G {{")?;
        writeln!(dot, "    rankdir=LR")?;
        writeln!(dot, "    fontcolor=darkslategray")?;
        writeln!(dot, "    node[shape=ellipse,color=darkslategray]")?;
        writeln!(dot, "    edge[headport=w,colorscheme=dark28]")?;

        writeln!(dot, "    labelloc=top")?;
        writeln!(dot, "    labeljust=l")?;
        writeln!(dot, "    label=\"FQDN RADIX TRIE\\l - {} leaves\\l - {} branching nodes\\l\"", self.leaf.len(), self.branching.len())?;

        // display all the branching nodes
        self.branching.iter().enumerate()
            .try_for_each(|(i,b)| {
                writeln!(dot, "{0:?} [label=\"[{0:?}] pos=-{1:?}\n[{2:?}] {3}\",peripheries={4}]",
                         i, b.pos, b.escape, self[b.escape].fqdn(), if b.escape.is_root_domain() {1} else {2})
            })?;

        // display node links
        writeln!(dot)?;
        writeln!(dot, "node[shape=none]")?;
        self.branching.iter().enumerate()
            .try_for_each(|(i,b)| {
                b.child.iter()
                    .enumerate()
                    .filter(|(_,&c)| c != b.escape)
                    .try_for_each(|(j,c)| {
                        let letter = if j == 0 {
                            '.'
                        } else {
                            ALPHABET.iter().enumerate()
                                .find(|(_, &x)| x as usize == j)
                                .map(|(c, _)| c as u8 as char)
                                .unwrap()
                                .to_ascii_lowercase()
                        };
                        if c.is_leaf() {
                            let c = LeafIndex::from(*c);
                            writeln!(dot, "{0:?}[label=\"[{0:?}] {1}\"]", c, self[c].fqdn())?;
                        }
                        writeln!(dot,"{0}->{1:?}[fontcolor={2},color={2},label=\"{3}\"]", i, c, 1+c.0.abs()%8, letter)
                    })
            })?;

        // end of writing...
        writeln!(dot, "}}")?;
        dot.flush()
    }
}