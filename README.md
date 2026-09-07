## Hiding text inside an image

Recently I was assigned with really interesting task, hiding text inside an image file. 

Well, basically, image is just an matrix of pixels, and matrix is just 2D array. 

*Now we have a question, wtf is a pixel?* \
Normally, pixel is consisted from 3 colors: red, green, blue and sometimes it even has alpha channel which defines the transparency level of the color. 

*How is the pixel represented in a display?* \
To put it simply, we just tell the computer how much red/green/blue we want in particular pixel in the range `0..255` each, and they are merged together to create one new unique color. 

For example purple is `rgb(191,0,255)` which has 74.9% red, 0% green and 100% blue in it. \  
Now imagine, if we just increase the red channel from 191 to 192, will it be noticable to human eye? Of course no. 

And we are talking about just 1 pixel here, how much would actual photo would change if we change its some pixels color to just 1% more or less? Almost none right?

*Okay, but why do we even need this information?*\
If we can change that one color channel, can't we just change it to some meaningful thing, letter for example. And can't we change more pixels the same way? We can! \
So we can technically **hide a text inside an image**! 

I know you are all interested so let's just move to code then) I use Rust for this case. \
First of all, let me show you the code so u can grasp the full picture: 
```rust    
static SRC: &str = "Reze.png";  
static DEST: &str = "result.png";
static TEXT: &str = "Denji";  
static PATTERN: &str = "!@#$";
    
// load the image  
let text: Vec<_> = format!("{PATTERN}{TEXT}{PATTERN}").into();
  
let mut image = open(SRC)?;
image
    .clone()
    .pixels() // take pixels
    .enumerate() // add indexing behaviour
    .take(text.len()) // only take starting pixels 
    .for_each(|(i, (x, y, Rgba([r, _g, b, a])))| {
        // replace green channel with hidden text 
        image.put_pixel(x, y, Rgba([r, text[i], b, a])); 
    });  
      
// save the image
image.save(DEST)?;

```

So we load the source file, extract the pixels, take only reasonable amount pixels we need to hide the text.\
Every pixel contatins x,y coordinates and rgba value. We replace the green channel in that pixels with our hidden text's characters one by one. 

 **Congratulations, you successfully hided the text inside image!**
 
 \
If you've noticed, there is also alpha channel (transparency), since I am using `.png` image format it is supported, but it might not be supported in other types like `.jpg` /`.jpeg` or `.bmp` 

You might be wondering, *what is the PATTERN constant doing here?* \
We just need it to successfully extract the hidden text. Because if we just use the indexes to indentify the hidden text, we may not be able to decode it if image saving logic does some magic behind the scenes. So using patterns before & after the data is more reliable option.

 **Decoding the data:**
```rust  
let pattern = PATTERN.as_bytes();
let image = open(DEST)?;  
  
// getting only green channels of pixels
let pixels = image
    .pixels()
    .map(|(_, _, Rgba([_r, g, _b, _a]))| g.clone())
    .collect::<Vec<u8>>();  
  
// creating text-length chunks from pixel array, and searching for
// a chunk that satisfies our hidden data
let result = pixels.chunks(text.len()).find_map(|x| {
    x.strip_circumfix(pattern, pattern)
});  
    
println!(
    "Hidden text is: {:?}",
    String::from_utf8_lossy(result.unwrap_or_default())
);
```  

Now, you successfully decoded the hidden text too. I hope it was fun and helpful post that can also encourage you to learn more about programming.

Credits goes to [@lambdajon](https://github.com/lambdajon) for giving me this idea.
