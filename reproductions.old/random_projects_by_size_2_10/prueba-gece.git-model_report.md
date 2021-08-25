# Model report for file:///tmp/top-repos-quality-repos-r4gls_q5/prueba-gece.git HEAD b55cff353745957e4d4ef1d7aa002052ab623e56

### Dump

```json
{'created_at': '2021-08-22 03:36:39',
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
 'size': '22.7 kB',
 'tags': [],
 'uuid': '8b6c8cee-0cd4-44c6-b31a-4562b8b63953',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-r4gls_q5/prueba-gece.git b55cff353745957e4d4ef1d7aa002052ab623e56

# javascript
33 rules, avg.len. 8.1
## train
PPCR: 0.921835
### report
macro
{'f1-score': 0.5254618186457741,
 'precision': 0.5684314355571696,
 'recall': 0.523011720148705,
 'support': 63484}
micro
{'f1-score': 0.9650148068804738,
 'precision': 0.9650148068804738,
 'recall': 0.9650148068804738,
 'support': 63484}
weighted
{'f1-score': 0.9596359465194952,
 'precision': 0.9617329230979198,
 'recall': 0.9650148068804738,
 'support': 63484}
### report_full
macro
{'f1-score': 0.44726185233560645,
 'precision': 0.5684314355571696,
 'recall': 0.4139411679241732,
 'support': 68867}
micro
{'f1-score': 0.9257655778951425,
 'precision': 0.9650148068804738,
 'recall': 0.8895842711313111,
 'support': 68867}
weighted
{'f1-score': 0.9065591668784004,
 'precision': 0.951802457793512,
 'recall': 0.8895842711313111,
 'support': 68867}
## test
PPCR: 0.929116
### report
macro
{'f1-score': 0.5469874825140806,
 'precision': 0.5862179646927482,
 'recall': 0.5373731169340327,
 'support': 28745}
micro
{'f1-score': 0.9769351191511567,
 'precision': 0.9769351191511567,
 'recall': 0.9769351191511567,
 'support': 28745}
weighted
{'f1-score': 0.9745284051807448,
 'precision': 0.9764554753547963,
 'recall': 0.9769351191511567,
 'support': 28745}
### report_full
macro
{'f1-score': 0.4648090990652592,
 'precision': 0.5862179646927482,
 'recall': 0.42606349964487167,
 'support': 30938}
micro
{'f1-score': 0.9410384866712465,
 'precision': 0.9769351191511567,
 'recall': 0.9076863404227811,
 'support': 30938}
weighted
{'f1-score': 0.9265941556746256,
 'precision': 0.9735826417779762,
 'recall': 0.9076863404227811,
 'support': 30938}
```

## javascript
### Summary
24 rules, avg.len. 8.0

| | |
|-|-|
|Min support|92|
|Max support|12755|
|Min confidence|0.9236055016517639|
|Max confidence|0.9997689723968506|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 12755.` |
| 2 | `  -1.label in {<space>}<br>	∧ -2.reserved not in {+}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.936. Support: 225.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.930. Support: 93.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 170.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 161.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 5263.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 1035.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 8589.` |
| 9 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 3848.` |
| 10 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2164.` |
| 11 | `  •••start_line ≥ 254<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.931. Support: 1349.` |
| 12 | `  •••start_line = 255<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.924. Support: 1237.` |
| 13 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved = ;<br>	∧ +1.reserved not in {;, if, }}<br>	∧ ^1.roles in {SCOPE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.983. Support: 150.` |
| 14 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +3.roles in {RIGHT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.944. Support: 135.` |
| 15 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.995. Support: 92.` |
| 16 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 656.` |
| 17 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 685.` |
| 18 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.928. Support: 314.` |
| 19 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 9<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 252.` |
| 20 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 271.` |
| 21 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 108.` |
| 22 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 485.` |
| 23 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {EXPRESSION} and not in {BLOCK, FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 120.` |
| 24 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 6605.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.0, "max_conf": 0.9997689723968506, "max_support": 12755, "min_conf": 0.9236055016517639, "min_support": 92, "num_rules": 24}}
```
</details>
