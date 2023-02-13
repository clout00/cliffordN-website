use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="container">
        <div id="navigation-bar">
          <nav>
            <ul>
              <li><a href="#" id="logo">{"LOGO"}</a></li>
              <li><a href="#">{"Home"}</a></li>
              <li><a href="#">{"About"}</a></li>
              <li><a href="#">{"Portfolio"}</a></li>
              <li><a href="#">{"Services"}</a></li>
              <li><a href="#">{"Contact"}</a></li>
              <a href="#" id={"menu-icon"}></a>
            </ul>
          </nav>
        </div>
      
        <div class="content">
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
          <p>{"Some text some text some text some text"}</p>
        </div>
      </div>
   }
}
