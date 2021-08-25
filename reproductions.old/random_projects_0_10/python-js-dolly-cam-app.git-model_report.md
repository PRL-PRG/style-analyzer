# Model report for file:///tmp/top-repos-quality-repos-v9ta4xr5/python-js-dolly-cam-app.git HEAD beeec04d8f47555a4ebd98f4b59d153dce57a8d2

### Dump

```json
{'created_at': '2021-08-22 18:20:49',
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
 'size': '18.0 kB',
 'tags': [],
 'uuid': '77d45872-47ee-4921-9948-f77df4214e30',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-v9ta4xr5/python-js-dolly-cam-app.git beeec04d8f47555a4ebd98f4b59d153dce57a8d2

# javascript
24 rules, avg.len. 5.3
## train
PPCR: 0.762317
### report
macro
{'f1-score': 0.1669459537843954,
 'precision': 0.16654131702916244,
 'recall': 0.1688173912452598,
 'support': 7845}
micro
{'f1-score': 0.8397705544933078,
 'precision': 0.8397705544933078,
 'recall': 0.8397705544933078,
 'support': 7845}
weighted
{'f1-score': 0.8063302999147417,
 'precision': 0.7801162483647355,
 'recall': 0.8397705544933078,
 'support': 7845}
### report_full
macro
{'f1-score': 0.1586630678878499,
 'precision': 0.16654131702916244,
 'recall': 0.15444291169745286,
 'support': 10291}
micro
{'f1-score': 0.7265108072342304,
 'precision': 0.8397705544933078,
 'recall': 0.6401710232241765,
 'support': 10291}
weighted
{'f1-score': 0.6340083424028992,
 'precision': 0.6374397316038038,
 'recall': 0.6401710232241765,
 'support': 10291}
## test
PPCR: 0.837121
### report
macro
{'f1-score': 0.16329527372211833,
 'precision': 0.15891686058811086,
 'recall': 0.16792207792207794,
 'support': 221}
micro
{'f1-score': 0.8280542986425339,
 'precision': 0.8280542986425339,
 'recall': 0.8280542986425339,
 'support': 221}
weighted
{'f1-score': 0.8048950279551127,
 'precision': 0.7829968784311174,
 'recall': 0.8280542986425339,
 'support': 221}
### report_full
macro
{'f1-score': 0.15587032267838208,
 'precision': 0.15891686058811086,
 'recall': 0.15329462989840348,
 'support': 264}
micro
{'f1-score': 0.754639175257732,
 'precision': 0.8280542986425339,
 'recall': 0.6931818181818182,
 'support': 264}
weighted
{'f1-score': 0.6956011026795756,
 'precision': 0.6992759197646017,
 'recall': 0.6931818181818182,
 'support': 264}
```

## javascript
### Summary
8 rules, avg.len. 4.4

| | |
|-|-|
|Min support|226|
|Max support|458|
|Min confidence|0.9811715483665466|
|Max confidence|0.998908281326294|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_depth': 10,
                     'min_samples_leaf': 105,
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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.998. Support: 330.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 244.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 458.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 323.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 226.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 292.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 231.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION}<br>	∧ ^2.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 239.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.375, "max_conf": 0.998908281326294, "max_support": 458, "min_conf": 0.9811715483665466, "min_support": 226, "num_rules": 8}}
```
</details>
