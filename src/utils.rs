pub fn dot_product(arr1: &Vec<f32>, arr2: &Vec<f32>) -> f32
{
	let mut result: f32 = 0.0;
	for (elem1, elem2) in arr1.iter().zip(arr2.iter()){
		result += elem1 * elem2;
	}
	return result;
}

pub fn vector_norm(vector: Vec<f32>) -> Vec<f32>
{
	let sum = 1.0 / vector.iter().fold(0f32, |sum, x| sum + (x * x)).sqrt();
	vector.iter().map(|x| x * sum).collect::<Vec<f32>>()
}