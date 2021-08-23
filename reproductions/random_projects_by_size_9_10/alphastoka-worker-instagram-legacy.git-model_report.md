# Model report for file:///tmp/top-repos-quality-repos-oasx7q99/alphastoka-worker-instagram-legacy.git HEAD c0f5f1f81154dec615a1c47deec2f41ec09d339b

### Dump

```json
{'created_at': '2021-08-20 16:39:40',
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
 'size': '18.1 kB',
 'tags': [],
 'uuid': '0a986c20-74ed-4f4b-ab36-c6d7c0865fca',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-oasx7q99/alphastoka-worker-instagram-legacy.git c0f5f1f81154dec615a1c47deec2f41ec09d339b

# javascript
6 rules, avg.len. 4.8
## train
PPCR: 0.679702
### report
macro
{'f1-score': 0.18079999793620125,
 'precision': 0.17749768741656213,
 'recall': 0.18422812266459368,
 'support': 4919}
micro
{'f1-score': 0.9064850579386055,
 'precision': 0.9064850579386055,
 'recall': 0.9064850579386055,
 'support': 4919}
weighted
{'f1-score': 0.8890730559570568,
 'precision': 0.8723193872451726,
 'recall': 0.9064850579386055,
 'support': 4919}
### report_full
macro
{'f1-score': 0.15638012468836077,
 'precision': 0.17749768741656213,
 'recall': 0.142819350233692,
 'support': 7237}
micro
{'f1-score': 0.7336294833826916,
 'precision': 0.9064850579386055,
 'recall': 0.6161392842337985,
 'support': 7237}
weighted
{'f1-score': 0.6572894134829679,
 'precision': 0.719812713553895,
 'recall': 0.6161392842337985,
 'support': 7237}
## test
PPCR: 0.593361
### report
macro
{'f1-score': 0.18110807113543093,
 'precision': 0.17822751322751323,
 'recall': 0.18414047012177853,
 'support': 143}
micro
{'f1-score': 0.9230769230769231,
 'precision': 0.9230769230769231,
 'recall': 0.9230769230769231,
 'support': 143}
weighted
{'f1-score': 0.9137621612313814,
 'precision': 0.904819254819255,
 'recall': 0.9230769230769231,
 'support': 143}
### report_full
macro
{'f1-score': 0.14462893386660203,
 'precision': 0.17822751322751323,
 'recall': 0.12772311212814647,
 'support': 241}
micro
{'f1-score': 0.6875000000000001,
 'precision': 0.9230769230769231,
 'recall': 0.5477178423236515,
 'support': 241}
weighted
{'f1-score': 0.6055804795361575,
 'precision': 0.7163790643043756,
 'recall': 0.5477178423236515,
 'support': 241}
```

## javascript
### Summary
3 rules, avg.len. 4.7

| | |
|-|-|
|Min support|100|
|Max support|1109|
|Min confidence|0.9598737359046936|
|Max confidence|0.9926470518112183|

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
                     'max_depth': 10,
                     'min_samples_leaf': 95,
                     'min_samples_split': 203,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 1109.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 204.` |
| 3 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.965. Support: 100.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.666666666666667, "max_conf": 0.9926470518112183, "max_support": 1109, "min_conf": 0.9598737359046936, "min_support": 100, "num_rules": 3}}
```
</details>
