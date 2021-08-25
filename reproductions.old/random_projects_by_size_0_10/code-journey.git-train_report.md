# Train report for javascript / file:///tmp/top-repos-quality-repos-8qoq8a52/code-journey.git HEAD 55fb871653646cd6ded47ab7d52cf37546739203

### Classification report

PPCR: 0.391

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 0.994| 0.637| 0.983| 0.770| 6040| 9428| 0.641 |
| `␣` | 0.955| 0.935| 0.255| 0.945| 0.403| 1346| 4931| 0.273 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 454| 0.106 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 1200| 0.023 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 20| 501| 0.040 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 7| 185| 0.038 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1121| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 703| 0.000 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 415| 0.000 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 201| 0.000 |
| `micro avg` | 0.970| 0.970| 0.379| 0.970| 0.545| 7489| 19139| 0.391 |
| `weighted avg` | 0.956| 0.970| 0.379| 0.963| 0.483| 7489| 19139| 0.391 |
| `macro avg` | 0.193| 0.193| 0.089| 0.193| 0.117| 7489| 19139| 0.391 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| "| ⏎␣⁺␣⁺| ⏎⏎| ⏎␣⁻␣⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3388 |6003 |37 |0 |0 |0 |0 |0 |0 |0 |0 |
|3585 |88 |1258 |0 |0 |0 |0 |0 |0 |0 |0 |
|1172 |20 |8 |0 |0 |0 |0 |0 |0 |0 |0 |
|1121 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|703 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|481 |14 |6 |0 |0 |0 |0 |0 |0 |0 |0 |
|415 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|406 |46 |2 |0 |0 |0 |0 |0 |0 |0 |0 |
|201 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|178 |1 |6 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Java/Uni-Ruse/Web Components - Spring Boot & React (Master's Degree)/forum_frontend/src/components/StatsPage.js | 36 |
| Java/Udemy/Go Full Stack with Spring Boot and React/todo-app/src/serviceWorker.js | 27 |
| Java/Uni-Ruse/Web Components - Spring Boot & React (Master's Degree)/forum_frontend/src/serviceWorker.js | 25 |
| Java/Uni-Ruse/Web Components - Spring Boot & React (Master's Degree)/forum_frontend/src/components/Users/Login.js | 12 |
| Java/Uni-Ruse/Web Components - Spring Boot & React (Master's Degree)/forum_frontend/src/components/Topics/TopicForm.js | 12 |
| JavaScript/JavaBrains/Introduction-to-JavaScript-for-Developers-Concepts-Snippet.js | 9 |
| Java/Uni-Ruse/Web Components - Spring Boot & React (Master's Degree)/forum_frontend/src/components/Replies/AddReplyPage.js | 8 |
| Java/Uni-Ruse/Web Components - Spring Boot & React (Master's Degree)/forum_frontend/src/services/Api.js | 8 |
| Java/Uni-Ruse/Web Components - Spring Boot & React (Master's Degree)/forum_frontend/src/components/Preloader.js | 6 |
| TypeScript/JavaBrains/TypeScript Basics/typescript-basics/out/generics.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.1927930444977632, "precision": 0.1927819490967856, "recall": 0.19284952717396653, "support": 7489}, "micro avg": {"f1-score": 0.9695553478435038, "precision": 0.9695553478435038, "recall": 0.9695553478435038, "support": 7489}, "weighted avg": {"f1-score": 0.9627203789667139, "precision": 0.9561109924940973, "recall": 0.9695553478435038, "support": 7489}, "\u2205": {"f1-score": 0.983131346216836, "precision": 0.9726182760855476, "recall": 0.9938741721854305, "support": 6040}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 20}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 7}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2423": {"f1-score": 0.944799098760796, "precision": 0.9552012148823082, "recall": 0.9346210995542348, "support": 1346}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 703}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1121}, "macro avg": {"f1-score": 0.11723042450507237, "precision": 0.1927819490967856, "recall": 0.08918410724768888, "support": 19139}, "micro avg": {"f1-score": 0.5453657803815533, "precision": 0.9695553478435038, "recall": 0.37938241287423585, "support": 19139}, "weighted avg": {"f1-score": 0.4828670576812008, "precision": 0.7252177385192123, "recall": 0.37938241287423585, "support": 19139}, "\u2205": {"f1-score": 0.7696153846153845, "precision": 0.9726182760855476, "recall": 0.636720407297412, "support": 9428}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1200}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 415}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 501}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 185}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 454}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 201}, "\u2423": {"f1-score": 0.4026888604353393, "precision": 0.9552012148823082, "recall": 0.2551206651794768, "support": 4931}},
  "ppcr": 0.39129526098542244
}
```
</details>
