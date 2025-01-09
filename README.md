# TP noté Rust

# Partie 1

## Question 2 :

Ce type correspond à une énumération fournie par le package `image`. 
Elle liste tous les types d'images disponibles sur le package.

Pour obtenir une image RGB, il faut utiliser la méthode `.into_rgb8()`.

## Question 3 :

Le canal alpha est conservé si on enregistre l'image de sortie en PNG sauf si on utilise la méthode `.into_rgb8()` ou toute autre méthode qui convertit l'image en RGB.

## Question 4 :

![img.png](rendu/img.png)

## Question 5 :

![images.jpeg](src/images/images.jpeg)
![output_quadrillage.png](src/images/output_quadrillage.png)


# Partie 2

## Question 6 :

On utilise le type `Luma` avec la méthode `.to_luma()` récupérer la luminositée du pixel.

## Question 7 :

```rust
fn pixel_luminositer(img: &RgbImage, x: u32, y: u32) -> u8 {
    let pixel = img.get_pixel(x, y);
    let Luma(luminosite_) = pixel.to_luma();
    return luminosite_[0];
}
```
