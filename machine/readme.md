# Supervised Learning

# 2.1 What is Machine Learning
> Machine learning is the science of getting computers to act without being specifically programmed.

This is done by implementing special algorithms that have the ability to detect patterns in data. From a developer point of view, this means creating a system that has access to relevant data, is able to feed the data to machine learning algorithms, and is able to take the output and redirect it to downstream processes and tasks.

Supervised learning Supervised learning happens when you pass both the input and the desired outputs to the system, and you want the resulting machine learning model to capture the relationship. Supervised learning is again divided into two subsections based on the type of the labels.

Supervised tasks when the target variable is continuous are termed as regression problems. 

For example, the price of a product can be any value. We should probably go for regression techniques when the prediction needs to be made on something that requires the prediction of a quantity of some sort. Quality of a regression model is generally measured using some form of error measures, that is, the difference between the target values and the predicted values.

Classification problems are different from regression problems in that the labels are discrete and finite. For example, we can categorize an email message as spam or ham. So, a problem is a classification problem when we are interested in the resulting bucket that a particular set of feature readings will fall into. Quality of a classification model is generally measured using an accuracy measure that is essentially the count of the number of times the model has been right.

Unsupervised learning In unsupervised algorithms, the labels or the target classes are not given. So, the goal of unsupervised learning is to attempt to find natual partitions of patterns.

*The best time for unsupervised learning is when the data on desired outcomes is not known, such as creating a new class of products that the business has never sold before.*

Reinforcement learning in reinforcement learning, specifically crafted agents are released in an environment, in which they learn to take actions based on some notion of rewards and punishments. Reinforcement learning has been applied to robotic movements and different classes of games with some success.

In this chapter we will be taking a look at creating models for different machine learning algorithms using Rust. We will first read a dataset from a csv file. This is a common dataset and will be representative of the types of data in the real world. Then we will look at the logic of popular algorithms, why they work, and how to implement them using some Rust machine learning packages such as rusty_machine.
 
We will also take a look at how to evaluate the accuracy of each model.

By the end of this chapter, you should have a fair understanding of how to create common machine learning models and implement them in Rust.

# 2.2 Dataset Specific Code
> Before going into regression algorithms and the associated code, let's build the surrounding boilerplate code. Look at the rusty_machine_regression package in the code that is shared with this book.

For showing usage of regression, we will be using the boston housing dataset.1 You can also download the data from the kaggle page for boston dataset.2 The dataset has 14 features and has 506 samples. The following is a description of the dataset.

In machine learning tasks, it is a good practice to shuffle the incoming dataset. Shuffling data serves the purpose of reducing variance and making sure that models remain general and overfit less. Shuffling helps in making sure that the train/test/validation samples of the dataset are representative of the overall distribution of the data.

# 2.3 Rusty_Machine Library

Rust_machine is a general-purpose machine learning library written entirely in Rust. The main aims of the rusty_machine are ease of use and speed without having to depend on a huge number of external libraries. The consistency in the api is achieved using the rust's trait system. It currently uses rulinalg for its linear algebra back end.

To use the rusty machine library, we will need to convert the data vectors into rulinalg supported Matrices. 