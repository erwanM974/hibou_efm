
<img src="./README_images/hibou_banner_v2.svg" alt="hibou banner" width="750">

# HIBOU EFM

HIBOU (for Holistic Interaction Behavioral Oracle Utility) provides utilities for the analysis of traces and 
multi-traces collected from the execution of Distributed Systems against interaction models.

This present version "hibou_efm" treats interaction models enriched with data and time via the use of a
third party software : [Diversity](https://projects.eclipse.org/projects/modeling.efm) (by the [CEA](http://www.cea.fr/)) 
which acts as a symbolic execution engine.

"hibou_efm" is an extension to data and time of "[hibou_label](https://github.com/erwanM974/hibou_label)". 
We invite you to familiarize yourself with "[hibou_label](https://github.com/erwanM974/hibou_label)" before going any further
with "hibou_efm".

This piece of software has been developed as part of my PhD thesis in 2018-2020 at the 
[CentraleSupelec](https://www.centralesupelec.fr/)
engineering school
(part of Université Paris-Saclay) 
in collaboration with the 
[CEA](http://www.cea.fr/) (Commissariat à l'énergie atomique et aux énergies alternatives).


# Principle

<img src="./README_images/data_time.png" alt="model with data and time" width="1000">


# Refining the execution of interaction models with symbolic execution

With those enriched models, we can refine the processes of interaction execution originally used for labelled interaction models.


## Example 1 : Multi-Trace analysis with PASS verdict

<img src="./README_images/exemple_data_pass.svg" alt="Multi-Trace analysis with PASS verdict" width="750">

## Example 2 : Multi-Trace analysis with FAIL verdict

<img src="./README_images/exemple_data_fail.svg" alt="Multi-Trace analysis with FAIL verdict" width="750">
 
## Example 3 : The scope operator to express variable scoping

<img src="./README_images/with_scope.svg" alt="The scope operator to express variable scoping" width="750">
