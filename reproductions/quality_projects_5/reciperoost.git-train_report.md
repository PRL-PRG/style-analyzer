# Train report for javascript / file:///tmp/top-repos-quality-repos-p4d2p68m/reciperoost.git HEAD dd397d9dad2548a66bc84a91c3d0d57d10cda5cb

### Classification report

PPCR: 0.532

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.973| 1.000| 0.742| 0.986| 0.842| 4290| 5784| 0.742 |
| `'` | 1.000| 1.000| 0.796| 1.000| 0.887| 747| 938| 0.796 |
| `␣` | 0.963| 0.910| 0.201| 0.936| 0.333| 536| 2422| 0.221 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 200| 200| 1.000 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 391| 0.123 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 28| 707| 0.040 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 15| 383| 0.039 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 206| 0.000 |
| `micro avg` | 0.976| 0.976| 0.519| 0.976| 0.678| 5864| 11031| 0.532 |
| `weighted avg` | 0.961| 0.976| 0.519| 0.969| 0.608| 5864| 11031| 0.532 |
| `macro avg` | 0.492| 0.489| 0.342| 0.490| 0.383| 5864| 11031| 0.532 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| "| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1494 |4290 |0 |0 |0 |0 |0 |0 |0 |
|1886 |48 |488 |0 |0 |0 |0 |0 |0 |
|191 |0 |0 |747 |0 |0 |0 |0 |0 |
|679 |24 |4 |0 |0 |0 |0 |0 |0 |
|368 |14 |1 |0 |0 |0 |0 |0 |0 |
|343 |34 |14 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |200 |0 |
|206 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| client/components/AllRecipes.js | 12 |
| client/components/EditRecipe.js | 9 |
| client/components/AddRecipe.js | 9 |
| server/api/recipes.js | 8 |
| server/auth/index.js | 8 |
| server/api/users.js | 7 |
| server/auth/google.js | 7 |
| client/components/RecipeDetails.js | 6 |
| script/encrypt-heroku-auth-token.js | 5 |
| server/api/users.spec.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 200}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 747}, "macro avg": {"f1-score": 0.49024614011306905, "precision": 0.4919142213098257, "recall": 0.48880597014925375, "support": 5864}, "micro avg": {"f1-score": 0.9762960436562074, "precision": 0.9762960436562074, "recall": 0.9762960436562074, "support": 5864}, "weighted avg": {"f1-score": 0.9685191232025943, "precision": 0.9611491338868668, "recall": 0.9762960436562074, "support": 5864}, "\u2205": {"f1-score": 0.9862068965517241, "precision": 0.9727891156462585, "recall": 1.0, "support": 4290}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 28}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 15}, "\u2423": {"f1-score": 0.9357622243528284, "precision": 0.9625246548323472, "recall": 0.9104477611940298, "support": 536}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 200}, "\u0027": {"f1-score": 0.8866468842729971, "precision": 1.0, "recall": 0.7963752665245203, "support": 938}, "macro avg": {"f1-score": 0.38269224807938607, "precision": 0.4919142213098257, "recall": 0.3424453607793222, "support": 11031}, "micro avg": {"f1-score": 0.6777153003847294, "precision": 0.9762960436562074, "recall": 0.5189919318284834, "support": 11031}, "weighted avg": {"f1-score": 0.6080102298930832, "precision": 0.8245713859941894, "recall": 0.5189919318284834, "support": 11031}, "\u2205": {"f1-score": 0.8416715715126546, "precision": 0.9727891156462585, "recall": 0.741701244813278, "support": 5784}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 707}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 206}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 391}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 383}, "\u2423": {"f1-score": 0.3332195288494367, "precision": 0.9625246548323472, "recall": 0.2014863748967795, "support": 2422}},
  "ppcr": 0.5315927839724413
}
```
</details>
