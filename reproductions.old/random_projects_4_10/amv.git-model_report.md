# Model report for file:///tmp/top-repos-quality-repos-tyhxrucq/amv.git HEAD 5aa9038237e56a40953a9239f11d08d16d83481d

### Dump

```json
{'created_at': '2021-08-21 21:44:33',
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
 'size': '12.8 kB',
 'tags': [],
 'uuid': '7c8c02e4-e1e7-41c1-8e8a-0c90e15a489b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-tyhxrucq/amv.git 5aa9038237e56a40953a9239f11d08d16d83481d

# javascript
2 rules, avg.len. 1.5
## train
PPCR: 0.348401
### report
macro
{'f1-score': 0.46202692405384815,
 'precision': 0.45588235294117646,
 'recall': 0.47388059701492535,
 'support': 632}
micro
{'f1-score': 0.9240506329113924,
 'precision': 0.9240506329113924,
 'recall': 0.9240506329113924,
 'support': 632}
weighted
{'f1-score': 0.9211473106490516,
 'precision': 0.9279597915115414,
 'recall': 0.9240506329113924,
 'support': 632}
### report_full
macro
{'f1-score': 0.26325703404221323,
 'precision': 0.45588235294117646,
 'recall': 0.18663422228866905,
 'support': 1814}
micro
{'f1-score': 0.4775143090760425,
 'precision': 0.9240506329113924,
 'recall': 0.3219404630650496,
 'support': 1814}
weighted
{'f1-score': 0.4610537527903218,
 'precision': 0.827647707374019,
 'recall': 0.3219404630650496,
 'support': 1814}
## test
PPCR: 0.529412
### report
macro
{'f1-score': 0.46693348580141036,
 'precision': 0.4622093023255814,
 'recall': 0.47596153846153844,
 'support': 180}
micro
{'f1-score': 0.9277777777777778,
 'precision': 0.9277777777777778,
 'recall': 0.9277777777777778,
 'support': 180}
weighted
{'f1-score': 0.9209940071575292,
 'precision': 0.9220284237726099,
 'recall': 0.9277777777777778,
 'support': 180}
### report_full
macro
{'f1-score': 0.3491666666666666,
 'precision': 0.4622093023255814,
 'recall': 0.2850877192982456,
 'support': 340}
micro
{'f1-score': 0.6423076923076922,
 'precision': 0.9277777777777778,
 'recall': 0.49117647058823527,
 'support': 340}
weighted
{'f1-score': 0.6133921568627452,
 'precision': 0.8375512995896033,
 'recall': 0.49117647058823527,
 'support': 340}
```

## javascript
### Summary
1 rules, avg.len. 2.0

| | |
|-|-|
|Min support|302|
|Max support|302|
|Min confidence|0.998344361782074|
|Max confidence|0.998344361782074|

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
| 1 | `  ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 302.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.0, "max_conf": 0.998344361782074, "max_support": 302, "min_conf": 0.998344361782074, "min_support": 302, "num_rules": 1}}
```
</details>
