# Model report for file:///tmp/top-repos-quality-repos-aqu8kliu/sgtk.git HEAD 1ab078d4c33e29143ee7c699420f300e8836cec5

### Dump

```json
{'created_at': '2021-08-20 23:26:09',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '19.0 kB',
 'tags': [],
 'uuid': '38a802ea-58ca-4199-98bb-ac8624297614',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-aqu8kliu/sgtk.git 1ab078d4c33e29143ee7c699420f300e8836cec5

# javascript
46 rules, avg.len. 9.5
## train
PPCR: 0.957238
### report
macro
{'f1-score': 0.7281475317621744,
 'precision': 0.7361237753510453,
 'recall': 0.7240172214094374,
 'support': 84616}
micro
{'f1-score': 0.9752529072515836,
 'precision': 0.9752529072515836,
 'recall': 0.9752529072515836,
 'support': 84616}
weighted
{'f1-score': 0.9726691103311776,
 'precision': 0.9707868970643353,
 'recall': 0.9752529072515836,
 'support': 84616}
### report_full
macro
{'f1-score': 0.6950732454912816,
 'precision': 0.7361237753510453,
 'recall': 0.6716360482467709,
 'support': 88396}
micro
{'f1-score': 0.9539453910711396,
 'precision': 0.9752529072515836,
 'recall': 0.9335490293678447,
 'support': 88396}
weighted
{'f1-score': 0.9455557461180457,
 'precision': 0.9605435579493399,
 'recall': 0.9335490293678447,
 'support': 88396}
## test
PPCR: 0.951234
### report
macro
{'f1-score': 0.7142292591202909,
 'precision': 0.7295982115949986,
 'recall': 0.7102133119841212,
 'support': 14259}
micro
{'f1-score': 0.970054000981836,
 'precision': 0.970054000981836,
 'recall': 0.970054000981836,
 'support': 14259}
weighted
{'f1-score': 0.9670265939198049,
 'precision': 0.9658249568267451,
 'recall': 0.970054000981836,
 'support': 14259}
### report_full
macro
{'f1-score': 0.6776134411578418,
 'precision': 0.7295982115949986,
 'recall': 0.6518487918493978,
 'support': 14990}
micro
{'f1-score': 0.945810113166262,
 'precision': 0.970054000981836,
 'recall': 0.9227484989993329,
 'support': 14990}
weighted
{'f1-score': 0.9364113495535304,
 'precision': 0.9545473696163395,
 'recall': 0.9227484989993329,
 'support': 14990}
```

## javascript
### Summary
30 rules, avg.len. 8.8

| | |
|-|-|
|Min support|95|
|Max support|17977|
|Min confidence|0.9339218735694885|
|Max confidence|0.9998743534088135|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 183,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.996. Support: 112.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 17977.` |
| 3 | `  -1.diff_offset ≥ 4<br>	∧ ^1.internal_type = ArrayExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 1.000. Support: 3980.` |
| 4 | `  -1.diff_offset ≤ 4<br>	∧ ^1.internal_type = ArrayExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 2003.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -4.reserved not in {.}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.975. Support: 495.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 378.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved not in {)}<br>	∧ +1.length ≥ 22<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.998. Support: 205.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ -4.length ≥ 27<br>	∧ +1.reserved not in {)}<br>	∧ +1.length ≥ 22<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 158.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 21<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 149.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 21<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 132.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 21<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 207.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.roles not in {IDENTIFIER, NUMBER}<br>	∧ -5.roles not in {BINARY}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.length ≤ 21<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 6738.` |
| 13 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 984.` |
| 14 | `  -1.roles in {CALL, EXPRESSION, STRING}<br>	∧ -3.length ≤ 4<br>	∧ -5.diff_offset ≤ 33<br>	∧ ^1.internal_type not in {ArrayExpression, VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.941. Support: 111.` |
| 15 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {ArrayExpression, VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 9512.` |
| 16 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 3701.` |
| 17 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.934. Support: 1869.` |
| 18 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.983. Support: 1668.` |
| 19 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1598.` |
| 20 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 869.` |
| 21 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 747.` |
| 22 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 346.` |
| 23 | `  -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 220.` |
| 24 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 137.` |
| 25 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.reserved = .<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {FOR} and not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 335.` |
| 26 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.reserved not in {.}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles in {FILE} and not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 178.` |
| 27 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.reserved not in {.}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {FILE, LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 95.` |
| 28 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.roles not in {RIGHT}<br>	∧ -5.diff_col ≥ 1<br>	∧ -5.reserved not in {.}<br>	∧ +1.internal_type = NumericLiteral<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 353.` |
| 29 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -2.length ≥ 2<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -4.roles not in {RIGHT}<br>	∧ -5.diff_col ≥ 1<br>	∧ -5.reserved not in {.}<br>	∧ +1.internal_type = NumericLiteral<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 135.` |
| 30 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ -4.roles not in {RIGHT}<br>	∧ -5.diff_col ≥ 1<br>	∧ -5.reserved not in {.}<br>	∧ +1.internal_type not in {NumericLiteral}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 6653.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.833333333333334, "max_conf": 0.9998743534088135, "max_support": 17977, "min_conf": 0.9339218735694885, "min_support": 95, "num_rules": 30}}
```
</details>
