# Model report for file:///tmp/top-repos-quality-repos-x01dm51g/cz-manager.git HEAD 706de5e0fc8dc8341b1e96d5b252cb4cd5d4db54

### Dump

```json
{'created_at': '2021-08-21 19:10:34',
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
 'size': '19.3 kB',
 'tags': [],
 'uuid': 'c1b5c008-ceb7-44b1-b616-a347fe1b73f8',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-x01dm51g/cz-manager.git 706de5e0fc8dc8341b1e96d5b252cb4cd5d4db54

# javascript
24 rules, avg.len. 7.9
## train
PPCR: 0.882349
### report
macro
{'f1-score': 0.6201064603931089,
 'precision': 0.6281874133033398,
 'recall': 0.6162074248296578,
 'support': 26099}
micro
{'f1-score': 0.955822062147975,
 'precision': 0.955822062147975,
 'recall': 0.955822062147975,
 'support': 26099}
weighted
{'f1-score': 0.9501473489456135,
 'precision': 0.9455314008359116,
 'recall': 0.955822062147975,
 'support': 26099}
### report_full
macro
{'f1-score': 0.507393392281233,
 'precision': 0.6281874133033398,
 'recall': 0.466476210482187,
 'support': 29579}
micro
{'f1-score': 0.8960810373935847,
 'precision': 0.955822062147975,
 'recall': 0.8433686061056831,
 'support': 29579}
weighted
{'f1-score': 0.8693696534660313,
 'precision': 0.9244077083049235,
 'recall': 0.8433686061056831,
 'support': 29579}
## test
PPCR: 0.905388
### report
macro
{'f1-score': 0.548072023410778,
 'precision': 0.5584429057649427,
 'recall': 0.5516277018681508,
 'support': 5579}
micro
{'f1-score': 0.9145008065961642,
 'precision': 0.9145008065961642,
 'recall': 0.9145008065961642,
 'support': 5579}
weighted
{'f1-score': 0.9100950924975975,
 'precision': 0.9098398540243686,
 'recall': 0.9145008065961642,
 'support': 5579}
### report_full
macro
{'f1-score': 0.47423099374310684,
 'precision': 0.5584429057649427,
 'recall': 0.44255364060180125,
 'support': 6162}
micro
{'f1-score': 0.8690912188058938,
 'precision': 0.9145008065961642,
 'recall': 0.827977929243752,
 'support': 6162}
weighted
{'f1-score': 0.8531629379891876,
 'precision': 0.8984658011425429,
 'recall': 0.827977929243752,
 'support': 6162}
```

## javascript
### Summary
13 rules, avg.len. 6.8

| | |
|-|-|
|Min support|116|
|Max support|5995|
|Min confidence|0.9267241358757019|
|Max confidence|0.9993849992752075|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.983. Support: 5995.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -3.internal_type = Identifier<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 301.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.internal_type = Identifier<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 957.` |
| 4 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 309.` |
| 5 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 3567.` |
| 6 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 1394.` |
| 7 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 813.` |
| 8 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 553.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 119.` |
| 10 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 255.` |
| 11 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 236.` |
| 12 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 4<br>	∧ -5.label in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 125.` |
| 13 | `  •••start_col ≤ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.927. Support: 116.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.846153846153846, "max_conf": 0.9993849992752075, "max_support": 5995, "min_conf": 0.9267241358757019, "min_support": 116, "num_rules": 13}}
```
</details>
