# Model report for file:///tmp/top-repos-quality-repos-5k4caepc/archer.git HEAD a85408ae014b2a84f76e8ec0bcc7c5ee0a9af2c1

### Dump

```json
{'created_at': '2021-08-29 12:45:16',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.2 kB',
 'tags': [],
 'uuid': '0cc94edc-34f3-4b29-8a46-535c9d08f0f6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5k4caepc/archer.git a85408ae014b2a84f76e8ec0bcc7c5ee0a9af2c1

# javascript
20 rules, avg.len. 9.6
## train
PPCR: 0.838495
### report
macro
{'f1-score': 0.5950913072129951,
 'precision': 0.6083706940142274,
 'recall': 0.5861255325427428,
 'support': 30024}
micro
{'f1-score': 0.9634625632827072,
 'precision': 0.9634625632827072,
 'recall': 0.9634625632827072,
 'support': 30024}
weighted
{'f1-score': 0.9612524518866664,
 'precision': 0.960295521555952,
 'recall': 0.9634625632827072,
 'support': 30024}
### report_full
macro
{'f1-score': 0.4456404382412096,
 'precision': 0.6083706940142274,
 'recall': 0.4063543135739334,
 'support': 35807}
micro
{'f1-score': 0.8788260849751637,
 'precision': 0.9634625632827072,
 'recall': 0.8078587985589409,
 'support': 35807}
weighted
{'f1-score': 0.8296646788466372,
 'precision': 0.9103164289866593,
 'recall': 0.8078587985589409,
 'support': 35807}
## test
PPCR: 0.772947
### report
macro
{'f1-score': 0.5901745590675089,
 'precision': 0.6163243896348612,
 'recall': 0.5720818289159365,
 'support': 7663}
micro
{'f1-score': 0.9200052198877724,
 'precision': 0.9200052198877724,
 'recall': 0.9200052198877724,
 'support': 7663}
weighted
{'f1-score': 0.9140102113352907,
 'precision': 0.9163948237427922,
 'recall': 0.9200052198877724,
 'support': 7663}
### report_full
macro
{'f1-score': 0.4730569111882301,
 'precision': 0.6163243896348612,
 'recall': 0.4191470204409342,
 'support': 9914}
micro
{'f1-score': 0.8021846731524152,
 'precision': 0.9200052198877724,
 'recall': 0.7111155941093403,
 'support': 9914}
weighted
{'f1-score': 0.7616266408069053,
 'precision': 0.8789502436304859,
 'recall': 0.7111155941093403,
 'support': 9914}
```

## javascript
### Summary
11 rules, avg.len. 7.5

| | |
|-|-|
|Min support|143|
|Max support|7077|
|Min confidence|0.9425531625747681|
|Max confidence|0.999119222164154|

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
                     'min_samples_leaf': 91,
                     'min_samples_split': 239,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 7077.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 143.` |
| 3 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.992. Support: 452.` |
| 4 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 3382.` |
| 5 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1703.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 1309.` |
| 7 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 484.` |
| 8 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 305.` |
| 9 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 235.` |
| 10 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 224.` |
| 11 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 2773.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.454545454545454, "max_conf": 0.999119222164154, "max_support": 7077, "min_conf": 0.9425531625747681, "min_support": 143, "num_rules": 11}}
```
</details>
