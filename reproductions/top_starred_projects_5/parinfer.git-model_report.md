# Model report for file:///tmp/top-repos-quality-repos-vvda93vz/parinfer.git HEAD 41c74d03534a5adbdcb7430fb666899e8dbf746d

### Dump

```json
{'created_at': '2021-08-29 22:35:22',
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
 'size': '19.2 kB',
 'tags': [],
 'uuid': '12fade85-de74-4c33-8693-d171616b92d6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-vvda93vz/parinfer.git 41c74d03534a5adbdcb7430fb666899e8dbf746d

# javascript
23 rules, avg.len. 7.0
## train
PPCR: 0.925784
### report
macro
{'f1-score': 0.7255684093741162,
 'precision': 0.7382748601257314,
 'recall': 0.7169303783083052,
 'support': 28990}
micro
{'f1-score': 0.9615384615384616,
 'precision': 0.9615384615384616,
 'recall': 0.9615384615384616,
 'support': 28990}
weighted
{'f1-score': 0.9591567642799556,
 'precision': 0.9574059133627014,
 'recall': 0.9615384615384616,
 'support': 28990}
### report_full
macro
{'f1-score': 0.619267826630233,
 'precision': 0.7382748601257314,
 'recall': 0.5694177039351758,
 'support': 31314}
micro
{'f1-score': 0.9244826213849827,
 'precision': 0.9615384615384616,
 'recall': 0.8901769176726065,
 'support': 31314}
weighted
{'f1-score': 0.9140176525366837,
 'precision': 0.9524960069621791,
 'recall': 0.8901769176726065,
 'support': 31314}
## test
PPCR: 0.878874
### report
macro
{'f1-score': 0.5623606832455386,
 'precision': 0.5650802978643675,
 'recall': 0.5657910925468658,
 'support': 4622}
micro
{'f1-score': 0.9357421029857205,
 'precision': 0.9357421029857205,
 'recall': 0.9357421029857205,
 'support': 4622}
weighted
{'f1-score': 0.9345579416845619,
 'precision': 0.9340479349982456,
 'recall': 0.9357421029857205,
 'support': 4622}
### report_full
macro
{'f1-score': 0.47308313820728654,
 'precision': 0.5650802978643675,
 'recall': 0.43275040031981005,
 'support': 5259}
micro
{'f1-score': 0.8754174678676249,
 'precision': 0.9357421029857205,
 'recall': 0.8223996957596501,
 'support': 5259}
weighted
{'f1-score': 0.8515733971433089,
 'precision': 0.8996555696620285,
 'recall': 0.8223996957596501,
 'support': 5259}
```

## javascript
### Summary
13 rules, avg.len. 5.7

| | |
|-|-|
|Min support|140|
|Max support|5351|
|Min confidence|0.929928719997406|
|Max confidence|0.9957746267318726|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 4841.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1745.` |
| 3 | `  -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 865.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION, MAP}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.987. Support: 436.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 3520.` |
| 6 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>⇒ y = ∅<br>Confidence: 0.996. Support: 815.` |
| 7 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {COMMENT}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 622.` |
| 8 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 529.` |
| 9 | `  -1.reserved not in {(, ,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {STRING} and not in {COMMENT}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.996. Support: 355.` |
| 10 | `  -1.reserved not in {(, ,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ∅<br>Confidence: 0.930. Support: 421.` |
| 11 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_col ≤ 78<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 492.` |
| 12 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.939. Support: 140.` |
| 13 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, STRING}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 5351.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.6923076923076925, "max_conf": 0.9957746267318726, "max_support": 5351, "min_conf": 0.929928719997406, "min_support": 140, "num_rules": 13}}
```
</details>
