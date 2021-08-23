# Model report for file:///tmp/top-repos-quality-repos-g7bm6_qh/structurefinder.git HEAD eed4e45d323f7deb639935e28d425574af000962

### Dump

```json
{'created_at': '2021-08-21 16:27:33',
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
 'size': '15.3 kB',
 'tags': [],
 'uuid': '00fbe1a1-7026-4edf-aea6-dd8753b47292',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-g7bm6_qh/structurefinder.git eed4e45d323f7deb639935e28d425574af000962

# javascript
9 rules, avg.len. 2.3
## train
PPCR: 0.431783
### report
macro
{'f1-score': 0.1305532074762844,
 'precision': 0.12020062262193013,
 'recall': 0.14285714285714285,
 'support': 1652}
micro
{'f1-score': 0.8414043583535109,
 'precision': 0.8414043583535109,
 'recall': 0.8414043583535109,
 'support': 1652}
weighted
{'f1-score': 0.768936264373031,
 'precision': 0.7079612942562834,
 'recall': 0.8414043583535109,
 'support': 1652}
### report_full
macro
{'f1-score': 0.1170822102425876,
 'precision': 0.12020062262193013,
 'recall': 0.11412151067323481,
 'support': 3826}
micro
{'f1-score': 0.5074844833880977,
 'precision': 0.8414043583535109,
 'recall': 0.36330371144798745,
 'support': 3826}
weighted
{'f1-score': 0.37272904358460973,
 'precision': 0.38265645152512,
 'recall': 0.36330371144798745,
 'support': 3826}
## test
PPCR: 0.401212
### report
macro
{'f1-score': 0.13440000000000002,
 'precision': 0.1268882175226586,
 'recall': 0.14285714285714285,
 'support': 331}
micro
{'f1-score': 0.8882175226586103,
 'precision': 0.8882175226586103,
 'recall': 0.8882175226586103,
 'support': 331}
weighted
{'f1-score': 0.8356350453172207,
 'precision': 0.788930367557799,
 'recall': 0.8882175226586103,
 'support': 331}
### report_full
macro
{'f1-score': 0.11666666666666667,
 'precision': 0.1268882175226586,
 'recall': 0.10796915167095116,
 'support': 825}
micro
{'f1-score': 0.5086505190311419,
 'precision': 0.8882175226586103,
 'recall': 0.3563636363636364,
 'support': 825}
weighted
{'f1-score': 0.38507070707070706,
 'precision': 0.4188080197747872,
 'recall': 0.3563636363636364,
 'support': 825}
```

## javascript
### Summary
2 rules, avg.len. 2.5

| | |
|-|-|
|Min support|157|
|Max support|311|
|Min confidence|0.9713375568389893|
|Max confidence|0.9887459874153137|

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
                     'max_depth': 5,
                     'max_features': 'auto',
                     'min_samples_leaf': 90,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 157.` |
| 2 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 311.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.5, "max_conf": 0.9887459874153137, "max_support": 311, "min_conf": 0.9713375568389893, "min_support": 157, "num_rules": 2}}
```
</details>
