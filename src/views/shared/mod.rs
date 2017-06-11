use maud::Markup;

pub fn root() -> Result<Markup, ::std::fmt::Error> {
  let name = "Lyra";
  let markup = html! {
    p { "Hi, " (name) "!" }
  };
  
  Ok(markup)
}