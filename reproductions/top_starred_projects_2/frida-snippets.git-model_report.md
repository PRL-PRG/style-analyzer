# Model report for file:///tmp/top-repos-quality-repos-ztfk92sj/frida-snippets.git HEAD e627f950c1aa26856b64aaff530d86599793e9fc

### Dump

```json
{'created_at': '2021-08-30 03:41:37',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '13.4 kB',
 'tags': [],
 'uuid': 'ff9f8f6b-cbbc-4992-931b-a472b4ed4015',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ztfk92sj/frida-snippets.git e627f950c1aa26856b64aaff530d86599793e9fc

# javascript
4 rules, avg.len. 4.5
## train
PPCR: 0.467003
### report
macro
{'f1-score': 0.19575551782682513,
 'precision': 0.1916874480465503,
 'recall': 0.2,
 'support': 1203}
micro
{'f1-score': 0.9584372402327515,
 'precision': 0.9584372402327515,
 'recall': 0.9584372402327515,
 'support': 1203}
weighted
{'f1-score': 0.9380968913313773,
 'precision': 0.9186019434649729,
 'recall': 0.9584372402327515,
 'support': 1203}
### report_full
macro
{'f1-score': 0.1716412355787123,
 'precision': 0.1916874480465503,
 'recall': 0.15539083557951483,
 'support': 2576}
micro
{'f1-score': 0.6102143424186293,
 'precision': 0.9584372402327515,
 'recall': 0.44759316770186336,
 'support': 2576}
weighted
{'f1-score': 0.4944013850908561,
 'precision': 0.5521431927427807,
 'recall': 0.44759316770186336,
 'support': 2576}
## test
PPCR: 0.417440
### report
macro
{'f1-score': 0.1940503432494279,
 'precision': 0.18844444444444444,
 'recall': 0.2,
 'support': 225}
micro
{'f1-score': 0.9422222222222222,
 'precision': 0.9422222222222222,
 'recall': 0.9422222222222222,
 'support': 225}
weighted
{'f1-score': 0.9141927281973048,
 'precision': 0.8877827160493826,
 'recall': 0.9422222222222222,
 'support': 225}
### report_full
macro
{'f1-score': 0.15880149812734082,
 'precision': 0.18844444444444444,
 'recall': 0.13721682847896438,
 'support': 539}
micro
{'f1-score': 0.5549738219895287,
 'precision': 0.9422222222222222,
 'recall': 0.39332096474953615,
 'support': 539}
weighted
{'f1-score': 0.45519167830564294,
 'precision': 0.540160791589363,
 'recall': 0.39332096474953615,
 'support': 539}
```

## javascript
### Summary
3 rules, avg.len. 4.0

| | |
|-|-|
|Min support|117|
|Max support|483|
|Min confidence|0.9699792861938477|
|Max confidence|0.9711538553237915|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.970. Support: 483.` |
| 2 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 260.` |
| 3 | `  -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 117.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.0, "max_conf": 0.9711538553237915, "max_support": 483, "min_conf": 0.9699792861938477, "min_support": 117, "num_rules": 3}}
```
</details>
