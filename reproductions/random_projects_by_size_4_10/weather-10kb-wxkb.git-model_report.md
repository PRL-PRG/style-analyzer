# Model report for file:///tmp/top-repos-quality-repos-av4ezdeu/weather-10kb-wxkb.git HEAD f3f17b417d411d50c633c25aea7ccbf624d80203

### Dump

```json
{'created_at': '2021-08-21 21:14:39',
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
 'size': '14.9 kB',
 'tags': [],
 'uuid': '1dc6333d-b270-487d-8d76-3e729588e9af',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-av4ezdeu/weather-10kb-wxkb.git f3f17b417d411d50c633c25aea7ccbf624d80203

# javascript
7 rules, avg.len. 3.9
## train
PPCR: 0.621056
### report
macro
{'f1-score': 0.6198131750882452,
 'precision': 0.6288942225673654,
 'recall': 0.6128905295526127,
 'support': 2047}
micro
{'f1-score': 0.9482169027845627,
 'precision': 0.9482169027845627,
 'recall': 0.9482169027845627,
 'support': 2047}
weighted
{'f1-score': 0.9406496739629502,
 'precision': 0.9353437591942808,
 'recall': 0.9482169027845627,
 'support': 2047}
### report_full
macro
{'f1-score': 0.4800209278131164,
 'precision': 0.6288942225673654,
 'recall': 0.4117596426614343,
 'support': 3296}
micro
{'f1-score': 0.7265581134194272,
 'precision': 0.9482169027845627,
 'recall': 0.5888956310679612,
 'support': 3296}
weighted
{'f1-score': 0.6843916915668258,
 'precision': 0.874112137292917,
 'recall': 0.5888956310679612,
 'support': 3296}
## test
PPCR: 0.627851
### report
macro
{'f1-score': 0.623287821397792,
 'precision': 0.6325396825396825,
 'recall': 0.6175120447676332,
 'support': 523}
micro
{'f1-score': 0.9330783938814532,
 'precision': 0.9330783938814532,
 'recall': 0.9330783938814532,
 'support': 523}
weighted
{'f1-score': 0.9153122521289373,
 'precision': 0.9002003095693344,
 'recall': 0.9330783938814532,
 'support': 523}
### report_full
macro
{'f1-score': 0.4548309646439221,
 'precision': 0.6325396825396825,
 'recall': 0.37344299421896415,
 'support': 833}
micro
{'f1-score': 0.7197640117994101,
 'precision': 0.9330783938814532,
 'recall': 0.5858343337334934,
 'support': 833}
weighted
{'f1-score': 0.6737870642971203,
 'precision': 0.8397301777853999,
 'recall': 0.5858343337334934,
 'support': 833}
```

## javascript
### Summary
6 rules, avg.len. 3.8

| | |
|-|-|
|Min support|104|
|Max support|621|
|Min confidence|0.9364508390426636|
|Max confidence|0.996268630027771|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.944. Support: 621.` |
| 2 | `  -1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.996. Support: 134.` |
| 3 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 147.` |
| 4 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 104.` |
| 5 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 417.` |
| 6 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 119.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.8333333333333335, "max_conf": 0.996268630027771, "max_support": 621, "min_conf": 0.9364508390426636, "min_support": 104, "num_rules": 6}}
```
</details>
