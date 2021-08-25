# Model report for file:///tmp/top-repos-quality-repos-t79wzl_9/blockdeliveryeos.git HEAD af8c3b962b37cc55a4f90c90145b3aeb2a1e16fc

### Dump

```json
{'created_at': '2021-08-24 22:18:46',
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
 'size': '16.0 kB',
 'tags': [],
 'uuid': 'd405904e-2b81-4b23-b4ed-7a5c3cc9f358',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-t79wzl_9/blockdeliveryeos.git af8c3b962b37cc55a4f90c90145b3aeb2a1e16fc

# javascript
6 rules, avg.len. 3.8
## train
PPCR: 0.748793
### report
macro
{'f1-score': 0.23763361946004036,
 'precision': 0.23424800008316182,
 'recall': 0.24112126908179538,
 'support': 4963}
micro
{'f1-score': 0.9413661092081402,
 'precision': 0.9413661092081402,
 'recall': 0.9413661092081402,
 'support': 4963}
weighted
{'f1-score': 0.9270823351710403,
 'precision': 0.9132345736403468,
 'recall': 0.9413661092081402,
 'support': 4963}
### report_full
macro
{'f1-score': 0.22492767994154245,
 'precision': 0.23424800008316182,
 'recall': 0.21690958543219213,
 'support': 6628}
micro
{'f1-score': 0.8061426969200242,
 'precision': 0.9413661092081402,
 'recall': 0.7048883524441762,
 'support': 6628}
weighted
{'f1-score': 0.7248532984251587,
 'precision': 0.7479834179764373,
 'recall': 0.7048883524441762,
 'support': 6628}
## test
PPCR: 0.455026
### report
macro
{'f1-score': 0.24686908402504731,
 'precision': 0.24772727272727274,
 'recall': 0.24609375,
 'support': 86}
micro
{'f1-score': 0.9883720930232558,
 'precision': 0.9883720930232558,
 'recall': 0.9883720930232558,
 'support': 86}
weighted
{'f1-score': 0.9883331470700789,
 'precision': 0.9885835095137421,
 'recall': 0.9883720930232558,
 'support': 86}
### report_full
macro
{'f1-score': 0.19777165280762404,
 'precision': 0.24772727272727274,
 'recall': 0.16459627329192547,
 'support': 189}
micro
{'f1-score': 0.6181818181818182,
 'precision': 0.9883720930232558,
 'recall': 0.4497354497354497,
 'support': 189}
weighted
{'f1-score': 0.5412970798376758,
 'precision': 0.6797498797498798,
 'recall': 0.4497354497354497,
 'support': 189}
```

## javascript
### Summary
3 rules, avg.len. 3.3

| | |
|-|-|
|Min support|155|
|Max support|1910|
|Min confidence|0.9484293460845947|
|Max confidence|0.9987593293190002|

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
| 1 | `  -1.reserved = .<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.999. Support: 403.` |
| 2 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.971. Support: 155.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.948. Support: 1910.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.3333333333333335, "max_conf": 0.9987593293190002, "max_support": 1910, "min_conf": 0.9484293460845947, "min_support": 155, "num_rules": 3}}
```
</details>
