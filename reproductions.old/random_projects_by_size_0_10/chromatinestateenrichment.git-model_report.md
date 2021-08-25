# Model report for file:///tmp/top-repos-quality-repos-ybybjd1v/chromatinestateenrichment.git HEAD cb02263d079c79abd44a20ca3cfb64fe837c3c5f

### Dump

```json
{'created_at': '2021-08-22 18:05:02',
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
 'size': '16.4 kB',
 'tags': [],
 'uuid': '56cc7df4-0b29-4f2e-8f04-250e5a658a47',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ybybjd1v/chromatinestateenrichment.git cb02263d079c79abd44a20ca3cfb64fe837c3c5f

# javascript
7 rules, avg.len. 5.1
## train
PPCR: 0.489254
### report
macro
{'f1-score': 0.5301865058799786,
 'precision': 0.5575276716298179,
 'recall': 0.5173220864826342,
 'support': 2117}
micro
{'f1-score': 0.9036372224846482,
 'precision': 0.9036372224846481,
 'recall': 0.9036372224846481,
 'support': 2117}
weighted
{'f1-score': 0.892741621147332,
 'precision': 0.8971186465596874,
 'recall': 0.9036372224846481,
 'support': 2117}
### report_full
macro
{'f1-score': 0.3300712213964785,
 'precision': 0.5575276716298179,
 'recall': 0.25431960121983926,
 'support': 4327}
micro
{'f1-score': 0.5937306021104903,
 'precision': 0.9036372224846481,
 'recall': 0.44210769586318466,
 'support': 4327}
weighted
{'f1-score': 0.5308037152128486,
 'precision': 0.8423857060138942,
 'recall': 0.44210769586318466,
 'support': 4327}
## test
PPCR: 0.413747
### report
macro
{'f1-score': 0.31685978064503695,
 'precision': 0.36627261281165,
 'recall': 0.3042933558558559,
 'support': 307}
micro
{'f1-score': 0.752442996742671,
 'precision': 0.752442996742671,
 'recall': 0.752442996742671,
 'support': 307}
weighted
{'f1-score': 0.721774115413517,
 'precision': 0.7334222947956055,
 'recall': 0.752442996742671,
 'support': 307}
### report_full
macro
{'f1-score': 0.19992574609951538,
 'precision': 0.36627261281165,
 'recall': 0.16567038482447966,
 'support': 742}
micro
{'f1-score': 0.44041944709246905,
 'precision': 0.752442996742671,
 'recall': 0.3113207547169811,
 'support': 742}
weighted
{'f1-score': 0.3606686239133972,
 'precision': 0.6884799119834307,
 'recall': 0.3113207547169811,
 'support': 742}
```

## javascript
### Summary
3 rules, avg.len. 5.7

| | |
|-|-|
|Min support|90|
|Max support|556|
|Min confidence|0.9379496574401855|
|Max confidence|0.9944444298744202|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.938. Support: 556.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 127.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {Identifier, StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {IF}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 90.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.666666666666667, "max_conf": 0.9944444298744202, "max_support": 556, "min_conf": 0.9379496574401855, "min_support": 90, "num_rules": 3}}
```
</details>
