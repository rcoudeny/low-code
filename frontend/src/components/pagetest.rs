use gloo_console::log;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub test: String,
}

#[function_component]
pub fn PageTest(props: &Props) -> Html {
    html! {
    <>
        {&props.test}
        <div>
            <ul>
                <li>{"Blakjsdflkasjdf"}</li>
                <li>{"alskfjaslkdjgsa"}</li>
                <li>{"asdlgkwhbgsldgh"}</li>
                <li>{"lskdgjdlgweglkj"}</li>
                <li>{"slgksjdkfjkw"}</li>
                <li>{"sdgkhe"}</li>
                <li>{"sdgkjsdlfkjslkdgjlk"}</li>
                <li>{"sdgkjsdlfkjslkdgjlk"}</li>
                <li>{"Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptates explicabo doloremque alias voluptatem? Est quibusdam odit magni ipsa sunt? Est dolor impedit quos quis perferendis ullam dolorem consequuntur animi architecto?"}</li>
                <li>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Repudiandae dolor corrupti, error corporis rerum ipsum atque itaque nulla modi quia minus debitis! Debitis repellat hic ex iusto, placeat numquam nobis!"}</li>
                <li>{"Lorem ipsum dolor sit amet consectetur adipisicing elit. Minus vitae earum, velit laboriosam voluptatibus eius enim accusantium quam eligendi ea cumque eum vero facere voluptate dicta ipsum quos delectus facilis."}</li>
                <li>{"Lorem ipsum dolor sit amet consectetur adipisicing elit. Facilis, necessitatibus dolor temporibus rem amet officiis corporis labore? Accusantium praesentium veniam quae porro recusandae nemo quisquam consectetur, voluptate, quo facere explicabo."}</li>
                <li>{"lslgdkhsdglkhsdglkhsalgkhaslkghlkasdhg"}</li>
                <li>{"sldgh;lablasdkhgalkdsjglk;dghalkhgdlhahg"}</li>
                <li>{"sldkghlkdwlkdjglkh"}</li>
                <li>{"lskhglkshglkashglkahsglkhgglshlgkhslkghlksdh    hlk hl   "}</li>
                <li>{"sdgshgdsfhsdhdfsh"}</li>
                <li>{"dsbfa"}</li>
                <li>{"sfbsdfhdsfhsdfh"}</li>
            </ul>
        </div>
    </>
    }
}
