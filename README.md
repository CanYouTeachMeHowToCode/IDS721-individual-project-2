# Individual Project #2: Kubernetes (or similar) Microservice in Rust
Build a functional web microservice in data engineering or machine learning engineering.


For the second project, I built a *Containerized* **Actix Microservice** for solving the [n-queens problem](https://www.quantamagazine.org/mathematician-answers-chess-problem-about-attacking-queens-20210921/) with different board sizes.

![a-A-solution-to-the-non-attacking-8-queens-problem-b-An-optimal-solution-to-the](https://user-images.githubusercontent.com/50161537/217409777-c2d1587a-5e19-4c74-a013-61f653a04687.png)

(image reference: https://www.researchgate.net/figure/a-A-solution-to-the-non-attacking-8-queens-problem-b-An-optimal-solution-to-the_fig1_278681097)

### Usage
> Run `make all` to build the docker image and start running the microservice (or directly run `cargo run` if the setup is already finished)

> After the initialization of the service, go to website `http://localhost:8080` if using a local machine or directly click the pop-out window when using Codespaces and the welcome page will appear

> Add suffix `/version` to the current URL and press 'enter' to check the version of the current microservice.

> Add suffix `/solve/[n]` to form all possible n-queens problem solutions for a specific board size `n`, where queens are represented by 'Q' and empty tiles are represented by '.'; e.g. the content showing on page `http://localhost:8080/solve/4` (using a local machine) is

```
.Q..
...Q
Q...
..Q.

..Q.
Q...
...Q
.Q..
```

> where the two boards above shows the only two valid solutions to the n-queens (4-queens here) problem with board size of 4.

> One can also use the command `docker pull yilunwu1/n-queens-microservice` to pull the project repository down and play it around by himself/herself. 

>In docker mode, run `docker run -it --rm -p 8080:8080 n-queens-microservice` to start running the Actix Microservice
