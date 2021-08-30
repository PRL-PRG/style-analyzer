# Model report for file:///tmp/top-repos-quality-repos-ghkylp30/odooapps.git HEAD 9ef6c85d8b47e8ca6a1d4527e41a7f134445cd2a

### Dump

```json
{'created_at': '2021-08-29 06:49:09',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '22.8 kB',
 'tags': [],
 'uuid': '8ccb823e-b7a4-4af2-871f-f3e48b621424',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ghkylp30/odooapps.git 9ef6c85d8b47e8ca6a1d4527e41a7f134445cd2a

# javascript
45 rules, avg.len. 9.9
## train
PPCR: 0.921328
### report
macro
{'f1-score': 0.3739833894450147,
 'precision': 0.37214216890781227,
 'recall': 0.3759325217523171,
 'support': 71812}
micro
{'f1-score': 0.9654932323288586,
 'precision': 0.9654932323288586,
 'recall': 0.9654932323288586,
 'support': 71812}
weighted
{'f1-score': 0.9599387070627231,
 'precision': 0.9544920481458898,
 'recall': 0.9654932323288586,
 'support': 71812}
### report_full
macro
{'f1-score': 0.3466035884810106,
 'precision': 0.37214216890781227,
 'recall': 0.3313267451262467,
 'support': 77944}
micro
{'f1-score': 0.9259595608857074,
 'precision': 0.9654932323288586,
 'recall': 0.889536077183619,
 'support': 77944}
weighted
{'f1-score': 0.903815051535537,
 'precision': 0.9272653583196339,
 'recall': 0.889536077183619,
 'support': 77944}
## test
PPCR: 0.805556
### report
macro
{'f1-score': 0.23997113997113997,
 'precision': 0.24222222222222223,
 'recall': 0.24249084249084252,
 'support': 29}
micro
{'f1-score': 0.8275862068965517,
 'precision': 0.8275862068965517,
 'recall': 0.8275862068965517,
 'support': 29}
weighted
{'f1-score': 0.8110912076429317,
 'precision': 0.8126436781609195,
 'recall': 0.8275862068965517,
 'support': 29}
### report_full
macro
{'f1-score': 0.20663780663780662,
 'precision': 0.24222222222222223,
 'recall': 0.1948717948717949,
 'support': 36}
micro
{'f1-score': 0.7384615384615385,
 'precision': 0.8275862068965517,
 'recall': 0.6666666666666666,
 'support': 36}
weighted
{'f1-score': 0.695045695045695,
 'precision': 0.7935185185185185,
 'recall': 0.6666666666666666,
 'support': 36}
```

## javascript
### Summary
28 rules, avg.len. 9.2

| | |
|-|-|
|Min support|99|
|Max support|11441|
|Min confidence|0.9544324278831482|
|Max confidence|0.9982798099517822|

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
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = '<br>Confidence: 0.955. Support: 99.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.987. Support: 11441.` |
| 3 | `  -1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 398.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = function<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 279.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, function}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 154.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, function}<br>	∧ +1.reserved = ]<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 115.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, function, {}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -4.length ≤ 21<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 5364.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, function, {}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 6<br>	∧ -4.label in {<space>}<br>	∧ -4.length ≤ 21<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 141.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, function, {}<br>	∧ -4.diff_offset ≤ 6<br>	∧ -4.label not in {<space>}<br>	∧ -4.length ≤ 21<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 580.` |
| 10 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 872.` |
| 11 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 8452.` |
| 12 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.972. Support: 1823.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 3167.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.960. Support: 768.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 3736.` |
| 16 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 141.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1029.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 1110.` |
| 19 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.989. Support: 222.` |
| 20 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≥ 106<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 178.` |
| 21 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 116.` |
| 22 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 400.` |
| 23 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = '<br>Confidence: 0.963. Support: 524.` |
| 24 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 153.` |
| 25 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 143.` |
| 26 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {FUNCTION} and not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 172.` |
| 27 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ +2.length ≤ 19<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 8043.` |
| 28 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 188.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.214285714285714, "max_conf": 0.9982798099517822, "max_support": 11441, "min_conf": 0.9544324278831482, "min_support": 99, "num_rules": 28}}
```
</details>
