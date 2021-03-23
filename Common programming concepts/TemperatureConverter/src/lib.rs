pub struct Fahrenheit(pub f64);
pub struct Celsius(pub f64);

impl From<Celsius> for Fahrenheit {
	fn from(celsius: Celsius) -> Self {
		Self(celsius.0 * (9.0 / 5.0) + 32.0)
	}
}

impl From<Fahrenheit> for Celsius {
	fn from(fahrenheit: Fahrenheit) -> Self {
		Self((fahrenheit.0 - 32.0) * (5.0 / 9.0)) 
	}
}

