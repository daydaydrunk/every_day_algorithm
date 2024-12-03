use rand::Rng;
use std::f64::consts::E;

pub struct Layer {
    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
    activations: Vec<f64>,
}

impl Layer {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let weights = (0..output_size)
            .map(|_| (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
            .collect();
        let biases = (0..output_size).map(|_| rng.gen_range(-1.0..1.0)).collect();

        Layer {
            weights,
            biases,
            activations: vec![0.0; output_size],
        }
    }
}

pub struct Neural {
    layers: Vec<Layer>,
    learning_rate: f64,
}

impl Neural {
    pub fn new(layer_sizes: &[usize]) -> Self {
        assert!(layer_sizes.len() >= 2);
        let mut layers = Vec::new();

        for i in 0..layer_sizes.len() - 1 {
            layers.push(Layer::new(layer_sizes[i], layer_sizes[i + 1]));
        }

        Neural {
            layers,
            learning_rate: 0.1,
        }
    }

    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + E.powf(-x))
    }

    fn sigmoid_derivative(x: f64) -> f64 {
        let sx = Self::sigmoid(x);
        sx * (1.0 - sx)
    }

    pub fn forward(&mut self, inputs: &[f64]) -> Vec<f64> {
        let mut current = inputs.to_vec();

        for layer in &mut self.layers {
            let mut new_outputs = vec![0.0; layer.biases.len()];

            for (j, bias) in layer.biases.iter().enumerate() {
                let sum: f64 = current
                    .iter()
                    .zip(&layer.weights[j])
                    .map(|(input, weight)| input * weight)
                    .sum();
                new_outputs[j] = Self::sigmoid(sum + bias);
            }

            layer.activations = new_outputs.clone();
            current = new_outputs;
        }

        current
    }

    pub fn train(&mut self, inputs: &[f64], targets: &[f64]) {
        let outputs = self.forward(inputs);
        let mut deltas = vec![Vec::new(); self.layers.len()];

        // Calculate output layer deltas
        let output_layer = self.layers.len() - 1;
        deltas[output_layer] = outputs
            .iter()
            .zip(targets)
            .map(|(output, target)| output * (1.0 - output) * (target - output))
            .collect();

        // Backpropagate deltas
        for layer_idx in (0..output_layer).rev() {
            let layer = &self.layers[layer_idx];
            let next_layer = &self.layers[layer_idx + 1];
            let mut layer_deltas = vec![0.0; layer.activations.len()];

            for j in 0..layer.activations.len() {
                let mut error = 0.0;
                for k in 0..next_layer.activations.len() {
                    error += next_layer.weights[k][j] * deltas[layer_idx + 1][k];
                }
                layer_deltas[j] = layer.activations[j] * (1.0 - layer.activations[j]) * error;
            }
            deltas[layer_idx] = layer_deltas;
        }

        // Update weights and biases
        let mut current = inputs.to_vec();
        for (layer_idx, layer) in self.layers.iter_mut().enumerate() {
            for j in 0..layer.biases.len() {
                layer.biases[j] += self.learning_rate * deltas[layer_idx][j];
                for (i, input) in current.iter().enumerate() {
                    layer.weights[j][i] += self.learning_rate * deltas[layer_idx][j] * input;
                }
            }
            current = layer.activations.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_training() {
        let mut neural = Neural::new(&[2, 4, 1]);
        let training_data = vec![
            (vec![0.0, 0.0], vec![0.0]),
            (vec![0.0, 1.0], vec![1.0]),
            (vec![1.0, 0.0], vec![1.0]),
            (vec![1.0, 1.0], vec![0.0]),
        ];

        // Train the network
        for _ in 0..10000 {
            for (inputs, targets) in &training_data {
                neural.train(inputs, targets);
            }
        }

        // Test the trained network
        for (inputs, expected) in &training_data {
            let output = neural.forward(inputs);
            println!(
                "Input: {:?}, Output: {:.4}, Expected: {}",
                inputs, output[0], expected[0]
            );
            assert!((output[0] - expected[0]).abs() < 0.1);
        }
    }
}
